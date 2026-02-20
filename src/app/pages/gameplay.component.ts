import { Component, OnInit } from "@angular/core";
import { OptionsComponent } from "../components/options/options.component";
import { OptionField } from "../_models/option-field.model";
import { BalancerService } from "../_services/balancer.service";
import { AutoTowerService } from "../_services/auto-tower.service";

@Component({
    selector: "app-gameplay",
    standalone: true,
    imports: [OptionsComponent],
    template: '<app-options [fields]="fields" (valueChange)="onValueChange($event)"></app-options>',
})
export class GameplayComponent implements OnInit {
    fields: OptionField[] = [
        {
            id: "automation",
            label: "Automation",
            type: "group",
            expanded: true,
            children: [
                { id: "balancer", label: "Balancer", type: "checkbox", value: false },
                // { id: "autoTower", label: "Auto Tower", type: "checkbox", value: false },
                // { id: "autoTowerKey", label: "Auto Tower Key", type: "key", value: "E" },
            ],
        },
    ];
    
    constructor(
        private balancerService: BalancerService,
        private autoTowerService: AutoTowerService
    ) {}

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
        if (event.id === "balancer") {
            try {
                await this.balancerService.setBalancerEnabled(event.value);
                console.log(`Balancer ${event.value ? "enabled" : "disabled"}`);
            } catch (error) {
                console.error("Failed to set balancer state:", error);
            }
        } else if (event.id === "autoTower") {
            try {
                await this.autoTowerService.setEnabled(event.value);
                console.log(`Auto Tower ${event.value ? "enabled" : "disabled"}`);
            } catch (error) {
                console.error("Failed to set auto tower state:", error);
            }
        } else if (event.id === "autoTowerKey") {
            try {
                await this.autoTowerService.setKey(event.value);
                console.log(`Auto Tower key set to: ${event.value}`);
            } catch (error) {
                console.error("Failed to set auto tower key:", error);
            }
        }
    }
}
