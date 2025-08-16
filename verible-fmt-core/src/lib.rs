pub mod formatter;
pub mod parser;

pub use formatter::VerilogFormatter;
pub use parser::VerilogParser;

pub type Result<T> = std::result::Result<T, anyhow::Error>;