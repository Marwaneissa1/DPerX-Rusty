import { Component, ViewContainerRef, ComponentRef, createComponent, EnvironmentInjector } from "@angular/core";

import { RouterOutlet, RouterLink } from "@angular/router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { GameStatusPopupComponent } from "./components/game-status-popup/game-status-popup.component";

@Component({
    selector: "app-root",
    standalone: true,
    imports: [RouterOutlet, RouterLink, GameStatusPopupComponent],
    templateUrl: "./app.component.html",
    styleUrl: "./app.component.css",
})
export class AppComponent {
    currentRoute = "combat";
    viewStatus = false;

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

    toggleGameStatus() {
        this.viewStatus = !this.viewStatus;
    }
}
