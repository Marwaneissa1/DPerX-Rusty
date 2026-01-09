import { Routes } from "@angular/router";
import { CombatComponent } from "./pages/combat.component";
import { VisualComponent } from "./pages/visual.component";
import { GameplayComponent } from "./pages/gameplay.component";

export const routes: Routes = [
    { path: "", redirectTo: "combat", pathMatch: "full" },
    { path: "combat", component: CombatComponent },
    { path: "visual", component: VisualComponent },
    { path: "gameplay", component: GameplayComponent },
];
