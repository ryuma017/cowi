#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    NotAscii,
    NotInteger,
    OverFlow,
}

impl ErrorKind {
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            Self::NotAscii => Some("Expect ASCII charactors but given invalid value"),
            Self::NotInteger => Some("Expect 32-bit signed integer but given invalid value"),
            Self::OverFlow => Some("Current memory value has overflowed"),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().unwrap_or_default().fmt(f)
    }
}
