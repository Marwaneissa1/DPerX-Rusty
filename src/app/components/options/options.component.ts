import { Component, Input, Output, EventEmitter, OnInit, OnDestroy } from "@angular/core";
import { Subscription } from "rxjs";

import { OptionItemComponent } from "../option-item/option-item.component";
import { OptionField } from "../../_models/option-field.model";
import { AttachStateService } from "../../_services/attach-state.service";

@Component({
    selector: "app-options",
    standalone: true,
    imports: [OptionItemComponent],
    templateUrl: "./options.component.html",
    styleUrls: ["./options.component.css"],
})
export class OptionsComponent implements OnInit, OnDestroy {
    @Input() fields: OptionField[] = [];
    @Output() valueChange = new EventEmitter<{ id: string; value: any }>();

    private attachSubscription?: Subscription;

    constructor(private attachStateService: AttachStateService) {}

    ngOnInit() {
        this.attachSubscription = this.attachStateService.isAttached$.subscribe((isAttached) => {
            this.updateFieldsDisabledState(isAttached);
        });
    }

    ngOnDestroy() {
        this.attachSubscription?.unsubscribe();
    }

    private updateFieldsDisabledState(isAttached: boolean) {
        const updateFields = (fields: OptionField[]) => {
            fields.forEach((field) => {
                field.disabled = !isAttached;
                if (field.children) {
                    updateFields(field.children);
                }
            });
        };
        updateFields(this.fields);
    }

    onValueChange(event: { id: string; value: any }) {
        if (this.attachStateService.isAttached) {
            this.valueChange.emit(event);
        }
    }
}
