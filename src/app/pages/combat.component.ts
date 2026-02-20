import { Component, OnInit } from "@angular/core";
import { OptionsComponent } from "../components/options/options.component";
import { OptionField } from "../_models/option-field.model";
import { AimbotService } from "../_services/aimbot.service";

@Component({
    selector: "app-combat",
    standalone: true,
    imports: [OptionsComponent],
    template: '<app-options [fields]="fields" (valueChange)="onValueChange($event)"></app-options>',
})
export class CombatComponent implements OnInit {
    fields: OptionField[] = [
        {
            id: "aimbot",
            label: "Aimbot",
            type: "group",
            expanded: true,
            children: [
                { id: "aimbot", label: "Aimbot", type: "checkbox", value: false },
                { id: "alwaysActive", label: "Always Active", type: "checkbox", value: false },
                { id: "aimbotKey", label: "Aimbot Key", type: "key", value: "F" },
                { id: "aimbotFov", label: "FOV Size", type: "slider", value: 90, min: 0, max: 360, step: 2 },
                {
                    id: "aimbotMaxDistance",
                    label: "Max Distance",
                    type: "slider",
                    value: 395,
                    min: 0,
                    max: 2000,
                    step: 50,
                },
            ],
        },
        {
            id: "predictions",
            label: "Predictions",
            type: "group",
            expanded: false,
            children: [
                { id: "predictionEnabled", label: "Enable Prediction", type: "checkbox", value: true },
                {
                    id: "predictionTime",
                    label: "Prediction Time (ms)",
                    type: "slider",
                    value: 50,
                    min: 0,
                    max: 500,
                    step: 10,
                },
            ],
        },
        {
            id: "autoActions",
            label: "Auto Actions",
            type: "group",
            expanded: false,
            children: [{ id: "autoFire", label: "Auto Fire", type: "checkbox", value: false }],
        },
        {
            id: "targeting",
            label: "Targeting",
            type: "group",
            expanded: false,
            children: [
                { id: "targetPriority", label: "Target Priority", type: "text", value: "Closest" },
                { id: "ignoreFrozen", label: "Ignore Frozen Tees", type: "checkbox", value: true },
            ],
        },
    ];

    constructor(private aimbotService: AimbotService) {}

    ngOnInit() {
        this.fields.forEach(group => {
            if (group.children) {
                group.children.forEach(field => {
                    if (field.id !== 'aimbotKey') {
                        this.onValueChange({ id: field.id, value: field.value });
                    }
                });
            }
        });

        setTimeout(() => {
            const aimbotKeyField = this.fields
                .flatMap(group => group.children || [])
                .find(field => field.id === 'aimbotKey');
            if (aimbotKeyField) {
                this.onValueChange({ id: aimbotKeyField.id, value: aimbotKeyField.value });
            }
        }, 100);
    }

    async onValueChange(event: { id: string; value: any }) {
        let response;

        console.log("Value change:", event);

        try {
            switch (event.id) {
                case "aimbot":
                    response = await this.aimbotService.setEnabled(event.value);
                    break;
                case "alwaysActive":
                    response = await this.aimbotService.setConfig({ alwaysActive: event.value });
                    break;
                case "aimbotKey":
                    response = await this.aimbotService.registerTriggerKey(event.value);
                    break;
                case "aimbotFov":
                    response = await this.aimbotService.setConfig({ fov: event.value });
                    break;
                case "aimbotMaxDistance":
                    response = await this.aimbotService.setConfig({ maxDistance: event.value });
                    break;
                case "predictionEnabled":
                    response = await this.aimbotService.setConfig({ predictionEnabled: event.value });
                    break;
                case "predictionTime":
                    response = await this.aimbotService.setConfig({ predictionTime: event.value });
                    break;
                case "ignoreFrozen":
                    response = await this.aimbotService.setConfig({ ignoreFrozen: event.value });
                    break;
                case "autoFire":
                    response = await this.aimbotService.setConfig({ autofire: event.value });
                    break;
            }
        } catch (error) {
            console.error("Failed to update aimbot config:", error);
        }

        console.log(response);
    }
}
