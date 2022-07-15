#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    InvalidValue,
    OverFlow,
}

impl ErrorKind {
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            Self::InvalidValue => Some("COW expect only ASCII charactors"),
            Self::OverFlow => Some("Current memory value has overflowed"),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().unwrap_or_default().fmt(f)
    }
}
