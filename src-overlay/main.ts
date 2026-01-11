import { invoke } from "@tauri-apps/api/core";

interface DrawLine {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
    color: [number, number, number, number];
    thickness: number;
}

interface DrawCircle {
    x: number;
    y: number;
    radius: number;
    color: [number, number, number, number];
    filled: boolean;
    thickness: number;
}

interface OverlayState {
    lines: DrawLine[];
    circles: DrawCircle[];
}

const canvas = document.getElementById("overlay-canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}

resizeCanvas();
window.addEventListener("resize", resizeCanvas);

function rgbaToString(color: [number, number, number, number]): string {
    return `rgba(${color[0]}, ${color[1]}, ${color[2]}, ${color[3] / 255})`;
}

async function render() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    try {
        const state = await invoke<OverlayState>("get_overlay_state");

        for (const line of state.lines) {
            ctx.strokeStyle = rgbaToString(line.color);
            ctx.lineWidth = line.thickness;
            ctx.beginPath();
            ctx.moveTo(line.x1, line.y1);
            ctx.lineTo(line.x2, line.y2);
            ctx.stroke();
        }

        for (const circle of state.circles) {
            ctx.strokeStyle = rgbaToString(circle.color);
            ctx.fillStyle = rgbaToString(circle.color);
            ctx.lineWidth = circle.thickness;
            ctx.beginPath();
            ctx.arc(circle.x, circle.y, circle.radius, 0, 2 * Math.PI);
            if (circle.filled) {
                ctx.fill();
            } else {
                ctx.stroke();
            }
        }
    } catch (error) {
        console.error("Failed to get overlay state:", error);
    }

    requestAnimationFrame(render);
}

render();
