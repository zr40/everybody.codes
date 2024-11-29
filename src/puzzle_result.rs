use std::fmt::Display;

pub(crate) enum Answer {
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    String(String),
    Multiline(String),
    #[allow(dead_code)]
    Todo,
    #[allow(dead_code)]
    SkipSlow,
    None,
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for Answer {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<i128> for Answer {
    fn from(value: i128) -> Self {
        Self::I128(value)
    }
}

impl From<isize> for Answer {
    fn from(value: isize) -> Self {
        Self::Isize(value)
    }
}

impl From<u16> for Answer {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for Answer {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(v) => v.fmt(f),
            Self::I64(v) => v.fmt(f),
            Self::I128(v) => v.fmt(f),
            Self::Isize(v) => v.fmt(f),
            Self::U16(v) => v.fmt(f),
            Self::U32(v) => v.fmt(f),
            Self::U64(v) => v.fmt(f),
            Self::Usize(v) => v.fmt(f),
            Self::String(v) => v.fmt(f),
            Self::Multiline(_) => "v v v v v v v v".fmt(f),
            Self::Todo => "TODO".fmt(f),
            Self::SkipSlow => "Skipped".fmt(f),
            Self::None => "".fmt(f),
        }
    }
}
