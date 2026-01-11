import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { AimbotConfig, AimbotStatus } from "../_models/aimbot-config.model";

@Injectable({
    providedIn: "root",
})
export class AimbotService {
    async setEnabled(enabled: boolean): Promise<string> {
        return await invoke<string>("set_aimbot_enabled", { enabled });
    }

    async setConfig(config: Partial<AimbotConfig>): Promise<string> {
        return await invoke<string>("set_aimbot_config", {
            fov: config.fov,
            edgeScan: config.edgeScan,
            maxDistance: config.maxDistance,
            alwaysActive: config.alwaysActive,
            predictionEnabled: config.predictionEnabled,
            predictionTime: config.predictionTime,
            ignoreFrozen: config.ignoreFrozen,
            autofire: config.autofire,
        });
    }

    async getStatus(): Promise<AimbotStatus> {
        return await invoke<AimbotStatus>("get_aimbot_status");
    }

    async registerTriggerKey(key: string): Promise<string> {
        return await invoke<string>("register_trigger_key", { key });
    }

    async unregisterTriggerKey(): Promise<string> {
        return await invoke<string>("unregister_trigger_key");
    }
}
