pub enum PreeditLoseFocusMode {
    Clear,
    Commit,
}

impl PreeditLoseFocusMode {
    pub fn value(&self) -> u32 {
        match self {
            Self::Clear => 0,
            Self::Commit => 1,
        }
    }
}
