import { Component, OnInit, OnDestroy, Output, EventEmitter } from "@angular/core";
import { CommonModule } from "@angular/common";
import { invoke } from "@tauri-apps/api/core";

interface GameStatus {
    local_player_id: number;
    online_players: number;
    player_pos: { x: number; y: number };
    aim_screen: { x: number; y: number };
    aim_world: { x: number; y: number };
}

@Component({
    selector: "app-game-status-popup",
    standalone: true,
    imports: [CommonModule],
    templateUrl: "./game-status-popup.component.html",
    styleUrl: "./game-status-popup.component.css",
})
export class GameStatusPopupComponent implements OnInit, OnDestroy {
    @Output() closePopup = new EventEmitter<void>();

    gameStatus: GameStatus | null = null;
    error: string | null = null;
    private intervalId: any;

    ngOnInit() {
        this.intervalId = setInterval(async () => {
            await this.updateStatus();
        }, 100);
    }

    ngOnDestroy() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
        }
    }

    async updateStatus() {
        try {
            this.gameStatus = await invoke<GameStatus>("get_game_status");
            this.error = null;
        } catch (err) {
            console.error("Error getting game status:", err);
            this.error = err as string;
            this.gameStatus = null;
        }
    }

    close() {
        this.closePopup.emit();
    }
}
