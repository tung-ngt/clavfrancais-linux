pub struct Capability;

impl Capability {
    pub const PREEDIT_TEXT: u32 = 1 << 0;
    pub const AUXILIARY_TEXT: u32 = 1 << 1;
    pub const LOOKUP_TABLE: u32 = 1 << 2;
    pub const FOCUS: u32 = 1 << 3;
    pub const PROPERTY: u32 = 1 << 4;
    pub const SURROUNDING_TEXT: u32 = 1 << 5;
}

pub struct KeyState;

impl KeyState {
    pub const CONTROL: u32 = 1 << 2;
    pub const ALT: u32 = 1 << 3;

    pub const MOD2: u32 = 1 << 4;
    pub const MOD4: u32 = 1 << 6;
    pub const MOD5: u32 = 1 << 7;

    pub const SUPER: u32 = 1 << 26;
    pub const HYPER: u32 = 1 << 27;
    pub const META: u32 = 1 << 28;

    pub const LOOSE_FOCUS: u32 = Self::CONTROL
        | Self::ALT
        | Self::SUPER
        | Self::MOD2
        | Self::MOD4
        | Self::MOD5
        | Self::SUPER
        | Self::HYPER
        | Self::META;

    pub const RELEASE: u32 = 1 << 30;
}
