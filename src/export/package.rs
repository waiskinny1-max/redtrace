use anyhow::{Context, Result};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::FileOptions;

pub fn export_zip(out: Option<PathBuf>) -> Result<()> {
    let workspace = crate::workspace::require_workspace()?;
    let reports_dir = workspace.join("reports");
    fs::create_dir_all(&reports_dir)?;

    let out = out.unwrap_or_else(|| reports_dir.join("redtrace-engagement.zip"));
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }

    let staging = reports_dir.join("export-staging");
    if staging.exists() {
        fs::remove_dir_all(&staging).with_context(|| format!("failed to clear {}", staging.display()))?;
    }
    fs::create_dir_all(&staging)?;

    stage_package(&workspace, &staging)?;
    write_zip(&staging, &out)?;
    fs::remove_dir_all(&staging).ok();

    println!("exported engagement package to {}", out.display());
    Ok(())
}

fn stage_package(workspace: &Path, staging: &Path) -> Result<()> {
    crate::report::markdown::generate(Some(staging.join("report.md")), crate::report::ReportProfile::Full)?;
    crate::report::html::generate(Some(staging.join("report.html")), crate::report::ReportProfile::Full)?;
    crate::evidence::commands::chain(Some(staging.join("chain-of-custody.md")))?;

    fs::write(staging.join("hashes.txt"), crate::export::manifest::build_hash_manifest()?)?;
    fs::write(staging.join("metadata.yaml"), serde_yaml::to_string(&crate::export::manifest::build_metadata()?)?)?;

    let timeline = workspace.join("timeline.jsonl");
    if timeline.exists() {
        fs::copy(&timeline, staging.join("timeline.jsonl"))?;
    } else {
        fs::write(staging.join("timeline.jsonl"), b"")?;
    }

    let evidence_src = workspace.join("evidence");
    let evidence_dst = staging.join("evidence");
    if evidence_src.exists() {
        copy_dir(&evidence_src, &evidence_dst)?;
    } else {
        fs::create_dir_all(&evidence_dst)?;
    }

    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(src)?;
        let target = dst.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target).with_context(|| format!("failed to copy {}", entry.path().display()))?;
        }
    }
    Ok(())
}

fn write_zip(source_dir: &Path, out: &Path) -> Result<()> {
    let file = fs::File::create(out).with_context(|| format!("failed to create {}", out.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let mut buffer = Vec::new();
    for entry in WalkDir::new(source_dir).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(source_dir)?;
        if relative.as_os_str().is_empty() {
            continue;
        }
        let name = zip_path(relative);

        if entry.file_type().is_dir() {
            let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);
            zip.add_directory(name, options)?;
        } else if entry.file_type().is_file() {
            let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o644);
            zip.start_file(name, options)?;
            let mut source = fs::File::open(path)?;
            source.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        }
    }

    zip.finish()?;
    Ok(())
}

fn zip_path(path: &Path) -> String {
    path.components().map(|part| part.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/")
}
