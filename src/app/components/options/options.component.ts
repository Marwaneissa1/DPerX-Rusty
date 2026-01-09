import { Component, Input } from "@angular/core";

import { OptionItemComponent } from "../option-item/option-item.component";
import { OptionField } from "../../_models/option-field.model";

@Component({
    selector: "app-options",
    standalone: true,
    imports: [OptionItemComponent],
    templateUrl: "./options.component.html",
    styleUrls: ["./options.component.css"],
})
export class OptionsComponent {
    @Input() fields: OptionField[] = [];
}
