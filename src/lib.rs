pub mod errs;
pub mod runtime;
pub use runtime::Runtime;
pub type Result<T> = std::result::Result<T, errs::Error>;
