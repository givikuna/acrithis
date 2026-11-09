#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Usage,
    Config,
    Io,
    Database,
    Hash,
    Graph,
    Validation,
    Permission,
    NotFound,
    Conflict,
}

impl Category {
    pub const fn prefix(&self) -> &'static str {
        match self {
            | Self::Usage => "EUSG",
            | Self::Config => "ECFG",
            | Self::Io => "EIO",
            | Self::Database => "EDB",
            | Self::Hash => "EHASH",
            | Self::Graph => "EGRF",
            | Self::Validation => "EVAL",
            | Self::Permission => "EPRM",
            | Self::NotFound => "ENFD",
            | Self::Conflict => "ECNF",
        }
    }

    pub const fn exit_code(&self) -> i32 {
        match self {
            | Self::Io | Self::Database => 1,
            | Self::Usage | Self::Config => 2,
            | Self::Validation => 3,
            | Self::Hash | Self::Graph => 4,
            | Self::NotFound => 5,
            | Self::Conflict => 6,
            | Self::Permission => 7,
        }
    }

    pub const fn is_retryable(&self) -> bool { matches!(self, Self::Io | Self::Validation) }
}
