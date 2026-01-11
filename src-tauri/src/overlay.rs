#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DrawLine {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: [u8; 4],
    pub thickness: f32,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DrawCircle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: [u8; 4],
    pub filled: bool,
    pub thickness: f32,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OverlayState {
    pub lines: Vec<DrawLine>,
    pub circles: Vec<DrawCircle>,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            circles: Vec::new(),
        }
    }
}
