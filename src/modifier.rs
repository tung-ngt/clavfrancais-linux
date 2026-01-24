pub const CONTROL_MASK: u32 = 1 << 2;
pub const ALT_MASK: u32 = 1 << 3;

pub const MOD2_MASK: u32 = 1 << 4;
pub const MOD4_MASK: u32 = 1 << 6;
pub const MOD5_MASK: u32 = 1 << 7;

pub const SUPER_MASK: u32 = 1 << 26;
pub const HYPER_MASK: u32 = 1 << 27;
pub const META_MASK: u32 = 1 << 28;

pub const LOOSE_FOCUS_MASK: u32 = CONTROL_MASK
    | ALT_MASK
    | SUPER_MASK
    | MOD2_MASK
    | MOD4_MASK
    | MOD5_MASK
    | SUPER_MASK
    | HYPER_MASK
    | META_MASK;

pub const RELEASE_MASK: u32 = 1 << 30;
