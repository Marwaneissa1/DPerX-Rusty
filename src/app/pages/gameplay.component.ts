import { Component } from "@angular/core";
import { OptionsComponent } from "../components/options/options.component";
import { OptionField } from "../_models/option-field.model";

@Component({
    selector: "app-gameplay",
    standalone: true,
    imports: [OptionsComponent],
    template: '<app-options [fields]="fields"></app-options>',
})
export class GameplayComponent {
    fields: OptionField[] = [
        {
            id: "automation",
            label: "Automation",
            type: "group",
            expanded: true,
            children: [
                { id: "balancer", label: "Balancer", type: "checkbox", value: false },
                { id: "autoTower", label: "Auto Tower", type: "checkbox", value: false },
                { id: "towerDelay", label: "Tower Delay (ms)", type: "integer", value: 50 },
                {
                    id: "balancerStrength",
                    label: "Balancer Strength",
                    type: "slider",
                    value: 50,
                    min: 0,
                    max: 100,
                    step: 5,
                },
            ],
        },
        {
            id: "spoofing",
            label: "Spoofing & Updates",
            type: "group",
            expanded: false,
            children: [
                { id: "localSpoofer", label: "Local Spoofer", type: "checkbox", value: true },
                { id: "autoUpdateOffsets", label: "Auto Update Offsets", type: "checkbox", value: true },
                { id: "spoofInterval", label: "Spoof Interval (s)", type: "integer", value: 30 },
                { id: "offsetCheckInterval", label: "Offset Check (min)", type: "integer", value: 60 },
            ],
        },
        {
            id: "settings",
            label: "Settings",
            type: "group",
            expanded: false,
            children: [
                { id: "keyBindings", label: "Custom Key Bindings", type: "checkbox", value: true },
                { id: "performanceMode", label: "Performance Mode", type: "checkbox", value: false },
                { id: "debugLogging", label: "Debug Logging", type: "checkbox", value: false },
                { id: "safeMode", label: "Safe Mode", type: "checkbox", value: true },
            ],
        },
    ];
}
