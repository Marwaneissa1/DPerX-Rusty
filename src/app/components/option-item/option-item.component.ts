import { Component, Input } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { OptionField } from "../../_models/option-field.model";
import { TextInputComponent } from "../text-input/text-input.component";
import { NumberInputComponent } from "../number-input/number-input.component";
import { SliderComponent } from "../slider/slider.component";
import { ToggleComponent } from "../toggle/toggle.component";
import { KeyPickerComponent } from "../key-picker/key-picker.component";

@Component({
    selector: "app-option-item",
    standalone: true,
    imports: [
        CommonModule,
        FormsModule,
        TextInputComponent,
        NumberInputComponent,
        SliderComponent,
        ToggleComponent,
        KeyPickerComponent,
    ],
    templateUrl: "./option-item.component.html",
    styleUrls: ["./option-item.component.css"],
})
export class OptionItemComponent {
    @Input() field!: OptionField;

    toggleGroup(): void {
        if (this.field.type === "group") {
            this.field.expanded = !this.field.expanded;
        }
    }
}
