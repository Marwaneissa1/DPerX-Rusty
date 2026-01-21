export interface OptionField {
    id: string;
    label: string;
    type: "text" | "integer" | "float" | "slider" | "checkbox" | "key" | "group" | "button";
    value?: string | number | boolean;
    min?: number;
    max?: number;
    step?: number;
    children?: OptionField[];
    expanded?: boolean;
    disabled?: boolean;
}
