use super::ReportProfile;
use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::PathBuf;

pub fn generate(out: Option<PathBuf>, profile: ReportProfile) -> Result<()> {
    let workspace = crate::workspace::require_workspace()?;
    let temp_markdown = workspace.join("reports").join(format!("report-{profile}.md"));
    crate::report::markdown::generate(Some(temp_markdown.clone()), profile.clone())?;
    let markdown = fs::read_to_string(&temp_markdown)?;

    let parser = Parser::new_ext(&markdown, Options::all());
    let mut body = String::new();
    html::push_html(&mut body, parser);

    let document = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>redtrace report</title>\n<style>body{{font-family:system-ui,sans-serif;max-width:980px;margin:40px auto;line-height:1.55}}table{{border-collapse:collapse;width:100%}}td,th{{border:1px solid #ddd;padding:6px}}code{{background:#f5f5f5;padding:2px 4px}}</style>\n</head>\n<body>\n{body}\n</body>\n</html>\n"
    );

    let out = out.unwrap_or_else(|| workspace.join("reports").join(format!("report-{profile}.html")));
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out, document)?;
    println!("generated HTML {profile} report at {}", out.display());
    Ok(())
}
