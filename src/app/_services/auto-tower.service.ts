import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";

@Injectable({
    providedIn: "root",
})
export class AutoTowerService {
    async setEnabled(enabled: boolean): Promise<string> {
        return await invoke<string>("set_auto_tower_enabled", { enabled });
    }

    async setKey(key: string): Promise<string> {
        return await invoke<string>("set_auto_tower_key", { key });
    }

    async getStatus(): Promise<{ enabled: boolean; trigger_key: string | null }> {
        return await invoke<{ enabled: boolean; trigger_key: string | null }>("get_auto_tower_status");
    }
}
