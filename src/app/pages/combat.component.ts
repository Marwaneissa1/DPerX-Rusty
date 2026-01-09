import { Component } from "@angular/core";
import { OptionsComponent } from "../components/options/options.component";
import { OptionField } from "../_models/option-field.model";

@Component({
    selector: "app-combat",
    standalone: true,
    imports: [OptionsComponent],
    template: '<app-options [fields]="fields"></app-options>',
})
export class CombatComponent {
    fields: OptionField[] = [
        {
            id: "aimbot",
            label: "Aimbot",
            type: "group",
            expanded: true,
            children: [
                { id: "aimbotWeapons", label: "Aimbot Weapons", type: "checkbox", value: false },
                { id: "aimbotHook", label: "Aimbot Hook", type: "checkbox", value: false },
                { id: "aimbotKey", label: "Aimbot Key", type: "key", value: "SHIFT" },
                { id: "showFov", label: "Show FOV Circle", type: "checkbox", value: true },
                { id: "weaponFov", label: "FOV Size", type: "slider", value: 90, min: 30, max: 180, step: 5 },
                { id: "maxDistance", label: "Max Distance", type: "slider", value: 500, min: 100, max: 1000, step: 50 },
            ],
        },
        {
            id: "predictions",
            label: "Predictions",
            type: "group",
            expanded: false,
            children: [
                { id: "predictionsEnabled", label: "Enable Predictions", type: "checkbox", value: true },
                { id: "predictionStrength", label: "Prediction Strength", type: "float", value: 1.0 },
                { id: "predictionKey", label: "Prediction Key", type: "key", value: "CTRL" },
                { id: "smoothing", label: "Smoothing", type: "slider", value: 50, min: 0, max: 100, step: 1 },
            ],
        },
        {
            id: "autoActions",
            label: "Auto Actions",
            type: "group",
            expanded: false,
            children: [
                { id: "autoFire", label: "Auto Fire", type: "checkbox", value: false },
                { id: "autoHammer", label: "Auto Hammer", type: "checkbox", value: false },
                { id: "autoFireKey", label: "Auto Fire Key", type: "key", value: "X" },
                { id: "reactionTime", label: "Reaction Time (ms)", type: "integer", value: 100 },
            ],
        },
        {
            id: "targeting",
            label: "Targeting",
            type: "group",
            expanded: false,
            children: [
                { id: "targetPriority", label: "Target Priority", type: "text", value: "Closest" },
                { id: "ignoreTeam", label: "Ignore Team", type: "checkbox", value: true },
                { id: "targetLock", label: "Target Lock", type: "checkbox", value: false },
            ],
        },
    ];
}
