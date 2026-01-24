pub const CONTROL_MASK: u32 = 1 << 2;
pub const ALT_MASK: u32 = 1 << 3;
pub const META_MASK: u32 = 1 << 28;
pub const RELEASE_MASK: u32 = 1 << 30;

pub const LOOSE_FOCUS_MASK: u32 = CONTROL_MASK | ALT_MASK | META_MASK;
