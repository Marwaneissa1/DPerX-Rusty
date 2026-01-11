use windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE, HWND, LPARAM, RECT};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, Process32FirstW, Process32NextW, MODULEENTRY32W,
    PROCESSENTRY32W, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowRect, GetWindowTextW, GetWindowThreadProcessId,
};

pub struct Process {
    handle: HANDLE,
    pid: u32,
    base_address: usize,
}

#[allow(dead_code)]
impl Process {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn base_address(&self) -> usize {
        self.base_address
    }

    pub fn from_process_name(process_name: &str) -> Result<Self, String> {
        let pid = find_pid_by_process_name(process_name)?;
        let base_address = get_module_base_address(pid, process_name)?;

        unsafe {
            let handle = OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION, false, pid)
                .map_err(|e| format!("Unable to open process {}: {:?}", pid, e))?;

            Ok(Process {
                handle,
                pid,
                base_address,
            })
        }
    }

    pub fn from_window_title(window_title: &str) -> Result<Self, String> {
        let pid = find_pid_by_window_title(window_title)?;

        let process_name = get_process_name(pid)?;
        let base_address = get_module_base_address(pid, &process_name)?;

        unsafe {
            let handle = OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION, false, pid)
                .map_err(|e| format!("Unable to open process {}: {:?}", pid, e))?;

            Ok(Process {
                handle,
                pid,
                base_address,
            })
        }
    }

    pub fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>, String> {
        unsafe {
            let mut buffer = vec![0u8; size];
            let mut bytes_read = 0;

            let result = ReadProcessMemory(
                self.handle,
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                size,
                Some(&mut bytes_read),
            );

            if result.is_ok() {
                buffer.truncate(bytes_read);
                Ok(buffer)
            } else {
                Err(format!("Error reading {} bytes from address 0x{:X}", size, address))
            }
        }
    }

    pub fn read<T: Copy>(&self, address: usize) -> Result<T, String> {
        let size = std::mem::size_of::<T>();
        let bytes = self.read_bytes(address, size)?;

        if bytes.len() < size {
            return Err(format!(
                "Insufficient bytes read: expected {}, got {}",
                size,
                bytes.len()
            ));
        }

        unsafe {
            let ptr = bytes.as_ptr() as *const T;
            Ok(*ptr)
        }
    }

    pub fn read_struct<T>(&self, address: usize) -> Result<T, String> {
        let size = std::mem::size_of::<T>();
        let mut buffer = vec![0u8; size];

        for i in 0..size {
            buffer[i] = self
                .read::<u8>(address + i)
                .map_err(|e| format!("Failed to read byte at offset {}: {}", i, e))?;
        }

        unsafe { Ok(std::ptr::read(buffer.as_ptr() as *const T)) }
    }

    pub fn write_bytes(&self, address: usize, data: &[u8]) -> Result<usize, String> {
        unsafe {
            let mut bytes_written = 0;

            let result = WriteProcessMemory(
                self.handle,
                address as *const _,
                data.as_ptr() as *const _,
                data.len(),
                Some(&mut bytes_written),
            );

            if result.is_ok() {
                Ok(bytes_written)
            } else {
                Err(format!("Error writing {} bytes to address 0x{:X}", data.len(), address))
            }
        }
    }

    pub fn write<T: Copy>(&self, address: usize, value: &T) -> Result<usize, String> {
        let size = std::mem::size_of::<T>();
        let bytes = unsafe { std::slice::from_raw_parts(value as *const T as *const u8, size) };
        self.write_bytes(address, bytes)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

fn get_process_name(pid: u32) -> Result<String, String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .map_err(|e| format!("Unable to create process snapshot: {:?}", e))?;

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &mut entry).is_err() {
            let _ = CloseHandle(snapshot);
            return Err("Unable to get first process".to_string());
        }

        loop {
            if entry.th32ProcessID == pid {
                let exe_name = String::from_utf16_lossy(
                    &entry.szExeFile[..entry
                        .szExeFile
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry.szExeFile.len())],
                );
                let _ = CloseHandle(snapshot);
                return Ok(exe_name);
            }

            if Process32NextW(snapshot, &mut entry).is_err() {
                break;
            }
        }

        let _ = CloseHandle(snapshot);
        Err(format!("Process with PID {} not found", pid))
    }
}

fn get_module_base_address(pid: u32, module_name: &str) -> Result<usize, String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)
            .map_err(|e| format!("Unable to create module snapshot: {:?}", e))?;

        let mut entry = MODULEENTRY32W {
            dwSize: std::mem::size_of::<MODULEENTRY32W>() as u32,
            ..Default::default()
        };

        if Module32FirstW(snapshot, &mut entry).is_err() {
            let _ = CloseHandle(snapshot);
            return Err("Unable to get first module".to_string());
        }

        loop {
            let mod_name = String::from_utf16_lossy(
                &entry.szModule[..entry
                    .szModule
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szModule.len())],
            );

            if mod_name.eq_ignore_ascii_case(module_name) {
                let base_addr = entry.modBaseAddr as usize;
                let _ = CloseHandle(snapshot);
                return Ok(base_addr);
            }

            if Module32NextW(snapshot, &mut entry).is_err() {
                break;
            }
        }

        let _ = CloseHandle(snapshot);
        Err(format!("Module '{}' not found", module_name))
    }
}

fn find_pid_by_process_name(process_name: &str) -> Result<u32, String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .map_err(|e| format!("Unable to create process snapshot: {:?}", e))?;

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &mut entry).is_err() {
            let _ = CloseHandle(snapshot);
            return Err("Unable to get first process".to_string());
        }

        loop {
            let exe_name = String::from_utf16_lossy(
                &entry.szExeFile[..entry
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len())],
            );

            if exe_name.eq_ignore_ascii_case(process_name) {
                let pid = entry.th32ProcessID;
                let _ = CloseHandle(snapshot);
                return Ok(pid);
            }

            if Process32NextW(snapshot, &mut entry).is_err() {
                break;
            }
        }

        let _ = CloseHandle(snapshot);
        Err(format!("Process '{}' not found", process_name))
    }
}

fn find_pid_by_window_title(window_title: &str) -> Result<u32, String> {
    struct EnumData {
        target_title: String,
        found_pid: Option<u32>,
    }

    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            let data = &mut *(lparam.0 as *mut EnumData);

            let mut buffer = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut buffer);

            if len > 0 {
                let title = String::from_utf16_lossy(&buffer[..len as usize]);

                if title.to_lowercase().contains(&data.target_title.to_lowercase()) {
                    let mut pid: u32 = 0;
                    GetWindowThreadProcessId(hwnd, Some(&mut pid));

                    if pid != 0 {
                        data.found_pid = Some(pid);
                        return false.into();
                    }
                }
            }

            true.into()
        }
    }

    let mut data = EnumData {
        target_title: window_title.to_string(),
        found_pid: None,
    };

    unsafe {
        let _ = EnumWindows(Some(enum_windows_callback), LPARAM(&mut data as *mut _ as isize));
    }

    data.found_pid
        .ok_or_else(|| format!("Window with title '{}' not found", window_title))
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WindowRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[allow(dead_code)]
pub fn get_window_rect_by_process_name(process_name: &str) -> Result<WindowRect, String> {
    let pid = find_pid_by_process_name(process_name)?;

    struct EnumData {
        target_pid: u32,
        found_hwnd: Option<HWND>,
    }

    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            let data = &mut *(lparam.0 as *mut EnumData);

            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));

            if pid == data.target_pid {
                data.found_hwnd = Some(hwnd);
                return false.into();
            }

            true.into()
        }
    }

    let mut data = EnumData {
        target_pid: pid,
        found_hwnd: None,
    };

    unsafe {
        let _ = EnumWindows(Some(enum_windows_callback), LPARAM(&mut data as *mut _ as isize));
    }

    let hwnd = data
        .found_hwnd
        .ok_or_else(|| format!("Window for process '{}' not found", process_name))?;

    unsafe {
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).map_err(|e| format!("Failed to get window rect: {:?}", e))?;

        Ok(WindowRect {
            x: rect.left,
            y: rect.top,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top,
        })
    }
}
