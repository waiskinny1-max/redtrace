use anyhow::Result;

pub fn run() -> Result<()> {
    println!("redtrace doctor");
    println!();
    println!("Binary:       OK");

    let workspace = crate::workspace::workspace_path();
    if !workspace.is_dir() {
        println!("Workspace:    MISSING");
        println!();
        println!("No redtrace workspace found.");
        println!("Run `redtrace init \"Engagement Name\"` first.");
        return Ok(());
    }

    println!("Workspace:    OK");
    check_path("Engagement", workspace.join("engagement.yaml").exists());
    check_path("Scope", workspace.join("scope.yaml").exists());
    check_path("Assets", workspace.join("assets.yaml").exists());
    check_path("Findings dir", workspace.join("findings").is_dir());
    check_path("Evidence dir", workspace.join("evidence").is_dir());
    check_path("Reports dir", workspace.join("reports").is_dir());
    check_path("Timeline", workspace.join("timeline.jsonl").exists());

    println!();
    println!("Next checks:");
    println!("  redtrace validate");
    println!("  redtrace evidence verify-all");
    Ok(())
}

fn check_path(label: &str, ok: bool) {
    let status = if ok { "OK" } else { "MISSING" };
    println!("{label:<13} {status}");
}
