export interface AimbotConfig {
    enabled: boolean;
    fov: number;
    edgeScan: boolean;
    maxDistance: number;
    alwaysActive: boolean;
    predictionEnabled: boolean;
    predictionTime: number;
    ignoreFrozen: boolean;
    autofire: boolean;
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
