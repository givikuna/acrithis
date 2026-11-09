pub mod category;
pub mod context;
pub mod core;
pub mod ext;
pub mod mappings;

pub use category::Category;
pub use context::ErrorContext;
pub use core::AcrithisError;
pub use ext::ResultExt;

pub type AcriResult<T> = std::result::Result<T, AcrithisError>;
