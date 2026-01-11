import { Component, ViewContainerRef, ComponentRef, createComponent, EnvironmentInjector, OnInit } from "@angular/core";

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
export class AppComponent implements OnInit {
    currentRoute = "combat";
    viewStatus = false;
    isAttached = false;

    async ngOnInit() {
        await this.checkAttachStatus();
    }

    async checkAttachStatus() {
        try {
            this.isAttached = await invoke<boolean>("get_attach_status");
        } catch (error) {
            console.error("Failed to get attach status:", error);
            this.isAttached = false;
        }
    }

    async minimizeWindow() {
        await getCurrentWindow().minimize();
    }

    async closeWindow() {
        await getCurrentWindow().close();
    }

    async attachToProcess() {
        try {
            if (this.isAttached) {
                // Unattach
                await invoke("unattach");
                this.isAttached = false;
                console.log("Successfully unattached from process");
            } else {
                // Attach
                await invoke("attach");
                this.isAttached = true;
                console.log("Successfully attached to process");
            }
        } catch (error) {
            console.error("Failed to toggle attach:", error);
        }
    }

    toggleGameStatus() {
        this.viewStatus = !this.viewStatus;
    }
}
