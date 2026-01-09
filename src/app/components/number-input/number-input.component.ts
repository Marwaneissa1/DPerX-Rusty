import { Component, Input, forwardRef } from "@angular/core";
import { NG_VALUE_ACCESSOR, ControlValueAccessor } from "@angular/forms";

@Component({
    selector: "app-number-input",
    standalone: true,
    template: `<input type="number" [value]="value" (input)="onInput($event)" [step]="step" class="option-input" />`,
    styleUrls: ["./number-input.component.css"],
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => NumberInputComponent),
            multi: true,
        },
    ],
})
export class NumberInputComponent implements ControlValueAccessor {
    @Input() value: number = 0;
    @Input() step: number = 1;
    onChange: any = () => {};
    onTouched: any = () => {};

    onInput(event: Event): void {
        const input = event.target as HTMLInputElement;
        this.value = parseFloat(input.value) || 0;
        this.onChange(this.value);
    }

    writeValue(value: number): void {
        this.value = value || 0;
    }

    registerOnChange(fn: any): void {
        this.onChange = fn;
    }

    registerOnTouched(fn: any): void {
        this.onTouched = fn;
    }
}
