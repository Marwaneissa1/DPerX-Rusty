use parking_lot::RwLock;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_MENU, VK_SHIFT};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

struct HookWrapper(Option<HHOOK>);
unsafe impl Send for HookWrapper {}
unsafe impl Sync for HookWrapper {}

static HOOK_HANDLE: RwLock<HookWrapper> = RwLock::new(HookWrapper(None));
static TRIGGER_KEY: RwLock<Option<i32>> = RwLock::new(None);
static KEY_STATE: RwLock<bool> = RwLock::new(false);

pub struct InputHook;

impl InputHook {
    pub fn register_trigger_key(vk_code: i32) -> Result<(), String> {
        *TRIGGER_KEY.write() = Some(vk_code);

        let hook_handle = HOOK_HANDLE.read().0;
        if hook_handle.is_none() {
            Self::install_hook()?;
        }

        Ok(())
    }

    pub fn unregister_trigger_key() -> Result<(), String> {
        *TRIGGER_KEY.write() = None;
        *KEY_STATE.write() = false;

        Self::uninstall_hook()?;
        Ok(())
    }

    pub fn is_trigger_key_pressed() -> bool {
        *KEY_STATE.read()
    }

    #[allow(dead_code)]
    pub fn check_key_state(vk_code: i32) -> bool {
        unsafe {
            let state = GetAsyncKeyState(vk_code);
            (state & 0x8000u16 as i16) != 0
        }
    }

    fn install_hook() -> Result<(), String> {
        unsafe {
            let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), None, 0)
                .map_err(|e| format!("Failed to install keyboard hook: {:?}", e))?;

            *HOOK_HANDLE.write() = HookWrapper(Some(hook));
            Ok(())
        }
    }

    fn uninstall_hook() -> Result<(), String> {
        let mut hook_handle = HOOK_HANDLE.write();

        if let Some(hook) = hook_handle.0 {
            unsafe {
                UnhookWindowsHookEx(hook).map_err(|e| format!("Failed to uninstall keyboard hook: {:?}", e))?;
            }
            *hook_handle = HookWrapper(None);
        }

        Ok(())
    }
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let kb_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode as i32;

        if let Some(trigger_key) = *TRIGGER_KEY.read() {
            if vk_code == trigger_key {
                match wparam.0 as u32 {
                    WM_KEYDOWN | WM_SYSKEYDOWN => {
                        *KEY_STATE.write() = true;
                    }
                    WM_KEYUP | WM_SYSKEYUP => {
                        *KEY_STATE.write() = false;
                    }
                    _ => {}
                }
            }
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

pub fn vk_code_from_string(key: &str) -> Option<i32> {
    match key.to_uppercase().as_str() {
        "SHIFT" => Some(VK_SHIFT.0 as i32),
        "CTRL" | "CONTROL" => Some(VK_CONTROL.0 as i32),
        "ALT" => Some(VK_MENU.0 as i32),
        "LSHIFT" => Some(0xA0),
        "RSHIFT" => Some(0xA1),
        "LCTRL" => Some(0xA2),
        "RCTRL" => Some(0xA3),
        "LALT" => Some(0xA4),
        "RALT" => Some(0xA5),
        "SPACE" => Some(0x20),
        "ENTER" => Some(0x0D),
        "TAB" => Some(0x09),
        "ESC" | "ESCAPE" => Some(0x1B),
        "BACKSPACE" => Some(0x08),
        "DELETE" => Some(0x2E),
        "INSERT" => Some(0x2D),
        "HOME" => Some(0x24),
        "END" => Some(0x23),
        "PAGEUP" => Some(0x21),
        "PAGEDOWN" => Some(0x22),
        "UP" => Some(0x26),
        "DOWN" => Some(0x28),
        "LEFT" => Some(0x25),
        "RIGHT" => Some(0x27),
        "F1" => Some(0x70),
        "F2" => Some(0x71),
        "F3" => Some(0x72),
        "F4" => Some(0x73),
        "F5" => Some(0x74),
        "F6" => Some(0x75),
        "F7" => Some(0x76),
        "F8" => Some(0x77),
        "F9" => Some(0x78),
        "F10" => Some(0x79),
        "F11" => Some(0x7A),
        "F12" => Some(0x7B),
        "0" => Some(0x30),
        "1" => Some(0x31),
        "2" => Some(0x32),
        "3" => Some(0x33),
        "4" => Some(0x34),
        "5" => Some(0x35),
        "6" => Some(0x36),
        "7" => Some(0x37),
        "8" => Some(0x38),
        "9" => Some(0x39),
        s if s.len() == 1 => {
            let ch = s.chars().next().unwrap();
            if ch.is_ascii_alphabetic() {
                Some(ch.to_ascii_uppercase() as i32)
            } else {
                None
            }
        }
        _ => None,
    }
}
