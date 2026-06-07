use super::ExportFormat;
use anyhow::Result;
use std::path::PathBuf;

pub fn run(format: ExportFormat, out: Option<PathBuf>) -> Result<()> {
    match format {
        ExportFormat::Zip => crate::export::package::export_zip(out),
    }
}
