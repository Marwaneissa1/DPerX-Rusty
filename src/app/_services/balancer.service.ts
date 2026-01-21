import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";

@Injectable({
    providedIn: "root",
})
export class BalancerService {
    async setBalancerEnabled(enabled: boolean): Promise<string> {
        return await invoke<string>("set_balancer_enabled", { enabled });
    }

    async getBalancerStatus(): Promise<{ enabled: boolean }> {
        return await invoke<{ enabled: boolean }>("get_balancer_status");
    }
}
