use crate::findings::model::{Confidence, FindingStatus};
use crate::findings::severity::Severity;
use crate::report::{ReportFormat, ReportProfile};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "redtrace")]
#[command(version, about = "Terminal-first red-team engagement tracker.")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a local redtrace workspace.
    Init {
        name: String,
        #[arg(long)]
        client: Option<String>,
        #[arg(long)]
        roe: Option<String>,
    },

    /// Print engagement status and object counts.
    Status,

    /// Check binary and local workspace health.
    Doctor,

    /// Validate completeness, scope hygiene, and evidence integrity.
    Validate {
        /// Return a failing exit code when critical/error issues are found.
        #[arg(long)]
        strict: bool,
    },

    /// Manage authorized scope and exclusions.
    Scope {
        #[command(subcommand)]
        command: ScopeCommands,
    },

    /// Manage asset inventory.
    Asset {
        #[command(subcommand)]
        command: AssetCommands,
    },

    /// Manage finding lifecycle.
    Finding {
        #[command(subcommand)]
        command: FindingCommands,
    },

    /// Manage evidence vault and hash verification.
    Evidence {
        #[command(subcommand)]
        command: EvidenceCommands,
    },

    /// Manage chronological activity timeline.
    Timeline {
        #[command(subcommand)]
        command: TimelineCommands,
    },

    /// Attach ATT&CK, OWASP, or CSF mappings to findings.
    Map {
        #[command(subcommand)]
        command: MapCommands,
    },

    /// Generate an engagement report.
    Report {
        #[arg(long, value_enum, default_value_t = ReportFormat::Markdown)]
        format: ReportFormat,
        #[arg(long, value_enum, default_value_t = ReportProfile::Full)]
        profile: ReportProfile,
        #[arg(long)]
        out: Option<PathBuf>,
    },

    /// Launch terminal operator console placeholder.
    Tui,
}

#[derive(Debug, Subcommand)]
enum ScopeCommands {
    Add {
        value: String,
        #[arg(long)]
        label: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    Exclude {
        value: String,
        #[arg(long)]
        reason: String,
    },
    List,
    Check {
        target: String,
    },
}

#[derive(Debug, Subcommand)]
enum AssetCommands {
    Add {
        hostname: String,
        #[arg(long)]
        ip: Option<String>,
        #[arg(long = "type", default_value = "unknown")]
        asset_type: String,
        #[arg(long)]
        environment: Option<String>,
        #[arg(long)]
        force_out_of_scope: bool,
    },
    List,
    Show { id: String },
    Tag { id: String, tag: String },
    Note { id: String, text: String },
}

#[derive(Debug, Subcommand)]
enum FindingCommands {
    New { title: String },
    List,
    Show { id: String },
    Set {
        id: String,
        #[arg(long, value_enum)]
        severity: Option<Severity>,
        #[arg(long)]
        asset: Option<String>,
        #[arg(long, value_enum)]
        status: Option<FindingStatus>,
        #[arg(long)]
        summary: Option<String>,
        #[arg(long)]
        impact: Option<String>,
        #[arg(long)]
        recommendation: Option<String>,
        #[arg(long, value_enum)]
        confidence: Option<Confidence>,
    },
    Note { id: String, text: String },
    Close {
        id: String,
        #[arg(long, value_enum, default_value_t = FindingStatus::Closed)]
        status: FindingStatus,
    },
}

#[derive(Debug, Subcommand)]
enum EvidenceCommands {
    Add {
        source: PathBuf,
        #[arg(long)]
        finding: Option<String>,
        #[arg(long)]
        asset: Option<String>,
        #[arg(long = "type")]
        evidence_type: Option<String>,
        #[arg(long)]
        note: Option<String>,
    },
    List,
    Show { id: String },
    Verify { id: String },
    VerifyAll,
    /// Print or export chain-of-custody data for all evidence.
    Chain {
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum TimelineCommands {
    Add {
        event: String,
        #[arg(long = "ref")]
        ref_id: Option<String>,
    },
    List,
    Export {
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum MapCommands {
    Attack {
        finding: String,
        #[arg(long)]
        tactic: String,
        #[arg(long)]
        technique: String,
    },
    Owasp {
        finding: String,
        #[arg(long = "id")]
        id: String,
    },
    Csf {
        finding: String,
        #[arg(long)]
        function: String,
    },
    List {
        finding: String,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, client, roe } => crate::engagement::commands::init(name, client, roe),
        Commands::Status => crate::engagement::commands::status(),
        Commands::Doctor => crate::doctor::run(),
        Commands::Validate { strict } => crate::validation::run(strict),
        Commands::Scope { command } => match command {
            ScopeCommands::Add { value, label, notes } => crate::scope::commands::add(value, label, notes),
            ScopeCommands::Exclude { value, reason } => crate::scope::commands::exclude(value, reason),
            ScopeCommands::List => crate::scope::commands::list(),
            ScopeCommands::Check { target } => crate::scope::commands::check(target),
        },
        Commands::Asset { command } => match command {
            AssetCommands::Add { hostname, ip, asset_type, environment, force_out_of_scope } => {
                crate::assets::commands::add(hostname, ip, asset_type, environment, force_out_of_scope)
            }
            AssetCommands::List => crate::assets::commands::list(),
            AssetCommands::Show { id } => crate::assets::commands::show(id),
            AssetCommands::Tag { id, tag } => crate::assets::commands::tag(id, tag),
            AssetCommands::Note { id, text } => crate::assets::commands::note(id, text),
        },
        Commands::Finding { command } => match command {
            FindingCommands::New { title } => crate::findings::commands::new(title),
            FindingCommands::List => crate::findings::commands::list(),
            FindingCommands::Show { id } => crate::findings::commands::show(id),
            FindingCommands::Set { id, severity, asset, status, summary, impact, recommendation, confidence } => {
                crate::findings::commands::set(id, severity, asset, status, summary, impact, recommendation, confidence)
            }
            FindingCommands::Note { id, text } => crate::findings::commands::note(id, text),
            FindingCommands::Close { id, status } => crate::findings::commands::close(id, status),
        },
        Commands::Evidence { command } => match command {
            EvidenceCommands::Add { source, finding, asset, evidence_type, note } => {
                crate::evidence::commands::add(source, finding, asset, evidence_type, note)
            }
            EvidenceCommands::List => crate::evidence::commands::list(),
            EvidenceCommands::Show { id } => crate::evidence::commands::show(id),
            EvidenceCommands::Verify { id } => crate::evidence::commands::verify(id).map(|_| ()),
            EvidenceCommands::VerifyAll => crate::evidence::commands::verify_all(),
            EvidenceCommands::Chain { out } => crate::evidence::commands::chain(out),
        },
        Commands::Timeline { command } => match command {
            TimelineCommands::Add { event, ref_id } => crate::timeline::commands::add(event, ref_id),
            TimelineCommands::List => crate::timeline::commands::list(),
            TimelineCommands::Export { out } => crate::timeline::commands::export(out),
        },
        Commands::Map { command } => match command {
            MapCommands::Attack { finding, tactic, technique } => crate::mappings::attack::add(finding, tactic, technique),
            MapCommands::Owasp { finding, id } => crate::mappings::owasp::add(finding, id),
            MapCommands::Csf { finding, function } => crate::mappings::csf::add(finding, function),
            MapCommands::List { finding } => crate::mappings::list(finding),
        },
        Commands::Report { format, profile, out } => crate::report::generate(format, profile, out),
        Commands::Tui => crate::tui::run(),
    }
}
