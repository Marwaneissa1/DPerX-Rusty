import { Component, Input, forwardRef } from "@angular/core";
import { NG_VALUE_ACCESSOR, ControlValueAccessor } from "@angular/forms";

@Component({
    selector: "app-slider",
    standalone: true,
    templateUrl: "./slider.component.html",
    styleUrls: ["./slider.component.css"],
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => SliderComponent),
            multi: true,
        },
    ],
})
export class SliderComponent implements ControlValueAccessor {
    @Input() value: number = 0;
    @Input() min: number = 0;
    @Input() max: number = 100;
    @Input() step: number = 1;
    onChange: any = () => {};
    onTouched: any = () => {};

    onInput(event: Event): void {
        const input = event.target as HTMLInputElement;
        this.value = parseFloat(input.value);
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
