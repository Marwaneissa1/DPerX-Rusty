import { Component, Input, Output, EventEmitter, DoCheck } from "@angular/core";
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
export class OptionItemComponent implements DoCheck {
    @Input() field!: OptionField;
    @Output() valueChange = new EventEmitter<{ id: string; value: any }>();

    private previousValue: any = undefined;
    private isInitialized = false;

    ngDoCheck() {
        if (this.field && this.field.type !== "group") {
            const currentValue = this.field.value;

            if (!this.isInitialized) {
                this.previousValue = this.deepCopy(currentValue);
                this.isInitialized = true;
                return;
            }

            if (!this.areValuesEqual(currentValue, this.previousValue)) {
                this.previousValue = this.deepCopy(currentValue);
                this.valueChange.emit({ id: this.field.id, value: currentValue });
            }
        }
    }

    private deepCopy(value: any): any {
        if (value === null || value === undefined) return value;
        if (typeof value !== "object") return value;
        return JSON.parse(JSON.stringify(value));
    }

    private areValuesEqual(a: any, b: any): boolean {
        if (a === b) return true;
        if (a === null || a === undefined || b === null || b === undefined) return false;
        if (typeof a !== typeof b) return false;
        if (typeof a !== "object") return a === b;
        return JSON.stringify(a) === JSON.stringify(b);
    }

    toggleGroup(): void {
        if (this.field.type === "group") {
            this.field.expanded = !this.field.expanded;
        }
    }

    onChildValueChange(event: { id: string; value: any }) {
        this.valueChange.emit(event);
    }

    onButtonClick(): void {
        if (this.field.type === "button") {
            this.valueChange.emit({ id: this.field.id, value: this.field.value });
        }
    }
}
