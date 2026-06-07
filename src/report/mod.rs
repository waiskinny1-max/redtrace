pub mod html;
pub mod markdown;
pub mod templates;

use anyhow::Result;
use clap::ValueEnum;
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
pub enum ReportFormat {
    Markdown,
    Html,
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Markdown => "markdown",
            Self::Html => "html",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ReportProfile {
    Executive,
    Technical,
    Full,
}

impl std::fmt::Display for ReportProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Executive => "executive",
            Self::Technical => "technical",
            Self::Full => "full",
        };
        write!(f, "{value}")
    }
}

pub fn generate(format: ReportFormat, profile: ReportProfile, out: Option<PathBuf>) -> Result<()> {
    match format {
        ReportFormat::Markdown => markdown::generate(out, profile),
        ReportFormat::Html => html::generate(out, profile),
    }
}
