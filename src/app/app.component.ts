import { Component, ViewContainerRef, ComponentRef, createComponent, EnvironmentInjector, OnInit } from "@angular/core";

import { RouterOutlet, RouterLink } from "@angular/router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { GameStatusPopupComponent } from "./components/game-status-popup/game-status-popup.component";
import { AttachStateService } from "./_services/attach-state.service";
import { AttachSplashComponent } from "./components/attach-splash/attach-splash.component";

@Component({
    selector: "app-root",
    standalone: true,
    imports: [RouterOutlet, RouterLink, GameStatusPopupComponent, AttachSplashComponent],
    templateUrl: "./app.component.html",
    styleUrl: "./app.component.css",
})
export class AppComponent implements OnInit {
    currentRoute = "combat";
    viewStatus = false;

    constructor(public attachStateService: AttachStateService) {}

    async ngOnInit() {
        await this.attachStateService.checkAttachStatus();
    }

    get isAttached(): boolean {
        return this.attachStateService.isAttached;
    }

    async minimizeWindow() {
        await getCurrentWindow().minimize();
    }

    async closeWindow() {
        await getCurrentWindow().close();
    }

    async attachToProcess() {
        try {
            await this.attachStateService.toggleAttach();
            console.log(this.isAttached ? "Successfully attached to process" : "Successfully unattached from process");
        } catch (error) {
            console.error("Failed to toggle attach:", error);
        }
    }

    toggleGameStatus() {
        this.viewStatus = !this.viewStatus;
    }
}
