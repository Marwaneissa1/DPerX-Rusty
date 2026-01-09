import { Component, Input, forwardRef } from "@angular/core";
import { NG_VALUE_ACCESSOR, ControlValueAccessor } from "@angular/forms";

@Component({
    selector: "app-text-input",
    standalone: true,
    template: `<input type="text" [value]="value" (input)="onInput($event)" class="option-input" />`,
    styleUrls: ["./text-input.component.css"],
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => TextInputComponent),
            multi: true,
        },
    ],
})
export class TextInputComponent implements ControlValueAccessor {
    @Input() value: string = "";
    onChange: any = () => {};
    onTouched: any = () => {};

    onInput(event: Event): void {
        const input = event.target as HTMLInputElement;
        this.value = input.value;
        this.onChange(this.value);
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
