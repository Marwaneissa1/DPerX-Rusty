import { Injectable } from "@angular/core";
import { BehaviorSubject, Observable } from "rxjs";
import { invoke } from "@tauri-apps/api/core";

@Injectable({
    providedIn: "root",
})
export class AttachStateService {
    private isAttachedSubject = new BehaviorSubject<boolean>(false);
    public isAttached$: Observable<boolean> = this.isAttachedSubject.asObservable();

    constructor() {
        this.checkAttachStatus();
        setInterval(() => this.checkAttachStatus(), 1000);
    }

    get isAttached(): boolean {
        return this.isAttachedSubject.value;
    }

    async checkAttachStatus(): Promise<void> {
        try {
            const status = await invoke<boolean>("get_attach_status");
            this.isAttachedSubject.next(status);
        } catch (error) {
            this.isAttachedSubject.next(false);
        }
    }

    async attach(): Promise<void> {
        await invoke("attach");
        this.isAttachedSubject.next(true);
    }

    async unattach(): Promise<void> {
        await invoke("unattach");
        this.isAttachedSubject.next(false);
    }

    async toggleAttach(): Promise<void> {
        if (this.isAttached) {
            await this.unattach();
        } else {
            await this.attach();
        }
    }
}
