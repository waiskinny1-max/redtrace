#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEvent {
    Quit,
    NextPanel,
    Open,
    AddNote,
    NewFinding,
    AddEvidence,
    GenerateReport,
    Search,
}
