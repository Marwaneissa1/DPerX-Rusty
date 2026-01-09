import { Component } from "@angular/core";
import { OptionsComponent } from "../components/options/options.component";
import { OptionField } from "../_models/option-field.model";

@Component({
    selector: "app-visual",
    standalone: true,
    imports: [OptionsComponent],
    template: '<app-options [fields]="fields"></app-options>',
})
export class VisualComponent {
    fields: OptionField[] = [
        {
            id: "espDisplay",
            label: "ESP Display",
            type: "group",
            expanded: true,
            children: [
                { id: "esp", label: "ESP Enabled", type: "checkbox", value: true },
                { id: "espBox", label: "ESP Box", type: "checkbox", value: true },
                { id: "espName", label: "ESP Name", type: "checkbox", value: true },
                { id: "espHealth", label: "ESP Health", type: "checkbox", value: true },
                { id: "espDistance", label: "ESP Distance", type: "checkbox", value: false },
                { id: "espWeapon", label: "ESP Weapon", type: "checkbox", value: true },
            ],
        },
        {
            id: "espStyling",
            label: "ESP Styling",
            type: "group",
            expanded: false,
            children: [
                {
                    id: "espMaxDistance",
                    label: "ESP Max Distance",
                    type: "slider",
                    value: 500,
                    min: 100,
                    max: 1000,
                    step: 50,
                },
                { id: "espOpacity", label: "ESP Opacity", type: "slider", value: 80, min: 0, max: 100, step: 5 },
                { id: "espBoxThickness", label: "Box Thickness", type: "float", value: 2.0 },
                { id: "espFontSize", label: "Font Size", type: "integer", value: 12 },
            ],
        },
        {
            id: "espColors",
            label: "Colors",
            type: "group",
            expanded: false,
            children: [
                { id: "espColorTeam", label: "Team Color", type: "text", value: "#00FF00" },
                { id: "espColorEnemy", label: "Enemy Color", type: "text", value: "#FF0000" },
            ],
        },
    ];
}
