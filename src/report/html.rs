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
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>redtrace {profile} report</title>\n<style>{}</style>\n</head>\n<body>\n<main class=\"report\">\n<div class=\"brand\">redtrace / authorized engagement report / {profile}</div>\n{}\n</main>\n</body>\n</html>\n",
        stylesheet(),
        body
    );

    let out = out.unwrap_or_else(|| workspace.join("reports").join(format!("report-{profile}.html")));
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out, document)?;
    println!("generated HTML {profile} report at {}", out.display());
    Ok(())
}

fn stylesheet() -> &'static str {
    r#"
:root {
  color-scheme: light;
  --bg: #f6f7f9;
  --paper: #ffffff;
  --ink: #16181d;
  --muted: #5c6470;
  --line: #d8dde6;
  --head: #111827;
  --soft: #eef2f7;
}
* { box-sizing: border-box; }
body {
  margin: 0;
  background: var(--bg);
  color: var(--ink);
  font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  line-height: 1.55;
}
.report {
  max-width: 1080px;
  margin: 32px auto;
  padding: 48px;
  background: var(--paper);
  border: 1px solid var(--line);
  box-shadow: 0 20px 50px rgba(15, 23, 42, 0.08);
}
.brand {
  margin-bottom: 32px;
  padding-bottom: 12px;
  border-bottom: 2px solid var(--head);
  color: var(--muted);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: .12em;
  text-transform: uppercase;
}
h1, h2, h3, h4 { color: var(--head); line-height: 1.25; }
h1 { margin: 0 0 8px; font-size: 36px; }
h2 { margin-top: 44px; padding-bottom: 8px; border-bottom: 1px solid var(--line); font-size: 24px; }
h3 { margin-top: 28px; font-size: 20px; }
h4 { margin-top: 20px; font-size: 16px; }
table {
  width: 100%;
  border-collapse: collapse;
  margin: 18px 0 28px;
  font-size: 14px;
}
th, td {
  border: 1px solid var(--line);
  padding: 9px 10px;
  vertical-align: top;
}
th {
  background: var(--soft);
  text-align: left;
  font-weight: 700;
}
tr:nth-child(even) td { background: #fbfcfe; }
code {
  background: var(--soft);
  border: 1px solid var(--line);
  border-radius: 4px;
  padding: 1px 5px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: .92em;
}
blockquote {
  margin: 18px 0;
  padding: 12px 16px;
  border-left: 4px solid var(--line);
  background: #fbfcfe;
  color: var(--muted);
}
@media print {
  body { background: #fff; }
  .report { max-width: none; margin: 0; padding: 0; border: 0; box-shadow: none; }
  h2 { break-after: avoid; }
  table, blockquote { break-inside: avoid; }
}
"#
}
