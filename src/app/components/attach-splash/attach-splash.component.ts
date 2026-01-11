import { Component, OnInit, OnDestroy } from "@angular/core";
import { CommonModule } from "@angular/common";
import { Subscription, interval } from "rxjs";
import { invoke } from "@tauri-apps/api/core";
import { AttachStateService } from "../../_services/attach-state.service";

interface GameStatus {
    process_found: boolean;
    window_found: boolean;
    process_name: string;
}

@Component({
    selector: "app-attach-splash",
    standalone: true,
    imports: [CommonModule],
    templateUrl: "./attach-splash.component.html",
    styleUrls: ["./attach-splash.component.css"],
})
export class AttachSplashComponent implements OnInit, OnDestroy {
    gameStatus: GameStatus = {
        process_found: false,
        window_found: false,
        process_name: "DDNet",
    };
    isAttaching = false;
    private statusCheckSubscription?: Subscription;

    constructor(public attachStateService: AttachStateService) {}

    ngOnInit() {
        this.checkGameStatus();
        this.statusCheckSubscription = interval(1000).subscribe(() => {
            this.checkGameStatus();
        });
    }

    ngOnDestroy() {
        this.statusCheckSubscription?.unsubscribe();
    }

    async checkGameStatus() {
        try {
            this.gameStatus = await invoke<GameStatus>("get_game_process_status");
        } catch (error) {
            this.gameStatus = {
                process_found: false,
                window_found: false,
                process_name: "DDNet",
            };
        }
    }

    async onAttach() {
        if (!this.gameStatus.process_found || this.isAttaching) return;

        this.isAttaching = true;
        try {
            await this.attachStateService.attach();
        } catch (error) {
            console.error("Failed to attach:", error);
        } finally {
            this.isAttaching = false;
        }
    }

    get canAttach(): boolean {
        return this.gameStatus.process_found && !this.isAttaching;
    }

    get statusMessage(): string {
        if (this.gameStatus.process_found && this.gameStatus.window_found) {
            return "DDNet process detected and ready";
        } else if (this.gameStatus.process_found) {
            return "DDNet process detected";
        } else {
            return "DDNet process not found";
        }
    }

    get statusIcon(): string {
        return this.gameStatus.process_found ? "✓" : "✗";
    }
}
