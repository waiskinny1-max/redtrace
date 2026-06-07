pub mod commands;
pub mod manifest;
pub mod package;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum ExportFormat {
    Zip,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Zip => "zip",
        };
        write!(f, "{value}")
    }
}
