import { Component, Input, forwardRef } from "@angular/core";
import { NG_VALUE_ACCESSOR, ControlValueAccessor } from "@angular/forms";

@Component({
    selector: "app-toggle",
    standalone: true,
    templateUrl: "./toggle.component.html",
    styleUrls: ["./toggle.component.css"],
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => ToggleComponent),
            multi: true,
        },
    ],
})
export class ToggleComponent implements ControlValueAccessor {
    @Input() value: boolean = false;
    onChange: any = () => {};
    onTouched: any = () => {};

    onToggle(): void {
        this.value = !this.value;
        this.onChange(this.value);
    }

    writeValue(value: boolean): void {
        this.value = value || false;
    }

    registerOnChange(fn: any): void {
        this.onChange = fn;
    }

    registerOnTouched(fn: any): void {
        this.onTouched = fn;
    }
}
