export interface AimbotConfig {
    enabled: boolean;
    fov: number;
    silent: boolean;
    hookVisible: boolean;
    edgeScan: boolean;
    maxDistance: number;
    alwaysActive?: boolean;
    predictionEnabled?: boolean;
    predictionTime?: number;
    targetPriority?: string;
    ignoreFrozen?: boolean;
}

export interface AimbotStatus {
    enabled: boolean;
    targetId: number;
    targetVisible: boolean;
    targetPos: {
        x: number;
        y: number;
    };
    aimPos: {
        x: number;
        y: number;
    };
}

export interface TriggerKeyConfig {
    key: string;
    registered: boolean;
}
