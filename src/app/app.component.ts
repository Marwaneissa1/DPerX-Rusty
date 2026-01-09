import { Component } from "@angular/core";

import { RouterOutlet, RouterLink } from "@angular/router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

@Component({
    selector: "app-root",
    standalone: true,
    imports: [RouterOutlet, RouterLink],
    templateUrl: "./app.component.html",
    styleUrl: "./app.component.css",
})
export class AppComponent {
    currentRoute = "combat";

    async minimizeWindow() {
        await getCurrentWindow().minimize();
    }

    async closeWindow() {
        await getCurrentWindow().close();
    }

    async attachToProcess() {
        try {
            await invoke("attach");
            console.log("Successfully attached to process");
        } catch (error) {
            console.error("Failed to attach to process:", error);
        }
    }
}
