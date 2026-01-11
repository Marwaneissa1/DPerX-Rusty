import { Component, OnInit, OnDestroy, Output, EventEmitter } from "@angular/core";
import { CommonModule } from "@angular/common";
import { invoke } from "@tauri-apps/api/core";

interface Player {
    id: number;
    gametick: number;
    pos: { x: number; y: number };
    vel: { x: number; y: number };
    frozen: boolean;
}

interface GameStatus {
    local_player_id: number;
    online_players: number;
    player_pos: { x: number; y: number };
    aim_screen: { x: number; y: number };
    aim_world: { x: number; y: number };
    players?: Player[];
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
    expandedPlayerId: number | null = null;
    private intervalId: any;

    ngOnInit() {
        this.fetchGameStatus();
        this.intervalId = setInterval(() => {
            this.fetchGameStatus();
        }, 100);
    }

    ngOnDestroy() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
        }
    }

    async fetchGameStatus() {
        try {
            this.gameStatus = await invoke("get_game_status");
        } catch (error) {}
    }

    togglePlayer(playerId: number) {
        if (this.expandedPlayerId === playerId) {
            this.expandedPlayerId = null;
        } else {
            this.expandedPlayerId = playerId;
        }
    }

    isPlayerExpanded(playerId: number): boolean {
        return this.expandedPlayerId === playerId;
    }

    onClose() {
        this.closePopup.emit();
    }
}
