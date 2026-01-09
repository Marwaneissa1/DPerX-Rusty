import { Component, Input, forwardRef } from "@angular/core";
import { NG_VALUE_ACCESSOR, ControlValueAccessor } from "@angular/forms";

@Component({
    selector: "app-key-picker",
    standalone: true,
    templateUrl: "./key-picker.component.html",
    styleUrls: ["./key-picker.component.css"],
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => KeyPickerComponent),
            multi: true,
        },
    ],
})
export class KeyPickerComponent implements ControlValueAccessor {
    @Input() value: string = "";
    isListening: boolean = false;
    onChange: any = () => {};
    onTouched: any = () => {};

    startListening(): void {
        this.isListening = true;
    }

    onKeyDown(event: KeyboardEvent): void {
        if (this.isListening) {
            event.preventDefault();
            this.value = event.key.toUpperCase();
            this.isListening = false;
            this.onChange(this.value);
        }
    }

    writeValue(value: string): void {
        this.value = value || "";
    }

    registerOnChange(fn: any): void {
        this.onChange = fn;
    }

    registerOnTouched(fn: any): void {
        this.onTouched = fn;
    }
}
