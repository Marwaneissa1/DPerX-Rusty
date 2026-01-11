import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";

export interface DrawLineParams {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
    color: [number, number, number, number];
    thickness: number;
}

export interface DrawCircleParams {
    x: number;
    y: number;
    radius: number;
    color: [number, number, number, number];
    filled: boolean;
    thickness: number;
}

export interface OverlayState {
    lines: DrawLineParams[];
    circles: DrawCircleParams[];
}

export interface WindowRect {
    x: number;
    y: number;
    width: number;
    height: number;
}

@Injectable({
    providedIn: "root",
})
export class OverlayService {
    private overlayWindow: WebviewWindow | null = null;
    private trackingInterval: any = null;

    async startOverlay(): Promise<void> {
        if (this.overlayWindow) {
            console.warn("Overlay already started");
            return;
        }

        try {
            await invoke<string>("start_overlay");

            const rect = await invoke<WindowRect>("get_window_rect");

            this.overlayWindow = new WebviewWindow("overlay", {
                url: "/overlay.html",
                title: "Overlay",
                width: rect.width,
                height: rect.height,
                x: rect.x,
                y: rect.y,
                decorations: false,
                transparent: true,
                alwaysOnTop: true,
                skipTaskbar: true,
                resizable: false,
            });

            this.startTracking();
        } catch (error) {
            console.error("Failed to start overlay:", error);
            throw error;
        }
    }

    private startTracking() {
        this.trackingInterval = setInterval(async () => {
            try {
                const rect = await invoke<WindowRect>("get_window_rect");
                if (this.overlayWindow) {
                    await this.overlayWindow.setPosition(new PhysicalPosition(rect.x, rect.y));
                    await this.overlayWindow.setSize(new PhysicalSize(rect.width, rect.height));
                }
            } catch (error) {}
        }, 100);
    }

    async stopOverlay(): Promise<void> {
        if (this.trackingInterval) {
            clearInterval(this.trackingInterval);
            this.trackingInterval = null;
        }

        if (this.overlayWindow) {
            await this.overlayWindow.close();
            this.overlayWindow = null;
        }
    }

    async getOverlayState(): Promise<OverlayState> {
        try {
            return await invoke<OverlayState>("get_overlay_state");
        } catch (error) {
            console.error("Failed to get overlay state:", error);
            throw error;
        }
    }

    async drawLine(params: DrawLineParams): Promise<void> {
        try {
            const result = await invoke<string>("draw_line", params as any);
            console.log(result);
        } catch (error) {
            console.error("Failed to draw line:", error);
            throw error;
        }
    }

    async drawCircle(params: DrawCircleParams): Promise<void> {
        try {
            const result = await invoke<string>("draw_circle", params as any);
            console.log(result);
        } catch (error) {
            console.error("Failed to draw circle:", error);
            throw error;
        }
    }

    async clearOverlay(): Promise<void> {
        try {
            const result = await invoke<string>("clear_overlay");
            console.log(result);
        } catch (error) {
            console.error("Failed to clear overlay:", error);
            throw error;
        }
    }

    async drawRedLine(): Promise<void> {
        await this.drawLine({
            x1: 100,
            y1: 100,
            x2: 300,
            y2: 300,
            color: [255, 0, 0, 255],
            thickness: 2.0,
        });
    }

    async drawBlueCircle(): Promise<void> {
        await this.drawCircle({
            x: 400,
            y: 300,
            radius: 50,
            color: [0, 0, 255, 255],
            filled: true,
            thickness: 2.0,
        });
    }

    async drawGreenCircleOutline(): Promise<void> {
        await this.drawCircle({
            x: 200,
            y: 200,
            radius: 75,
            color: [0, 255, 0, 200],
            filled: false,
            thickness: 3.0,
        });
    }

    async runDemo(): Promise<void> {
        await this.startOverlay();

        await new Promise((resolve) => setTimeout(resolve, 500));

        await this.drawRedLine();
        await this.drawBlueCircle();
        await this.drawGreenCircleOutline();
    }
}
