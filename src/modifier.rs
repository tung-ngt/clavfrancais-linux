pub const SHIFT_MASK: u32 = 1 << 0;
pub const LOCK_MASK: u32 = 1 << 1;
pub const CONTROL_MASK: u32 = 1 << 2;
pub const ALT_MASK: u32 = 1 << 3;
pub const META_MASK: u32 = 1 << 28;
pub const RELEASE_MASK: u32 = 1 << 30;

pub const LOOSE_FOCUS_MASK: u32 = CONTROL_MASK | ALT_MASK | META_MASK;

pub const CAP_KEY: u32 = 65509;
pub const SHIFT_KEY: u32 = 65505;
