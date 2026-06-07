use super::model::Engagement;
use crate::workspace::{read_yaml, require_workspace, write_yaml};
use anyhow::Result;

pub fn engagement_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(require_workspace()?.join("engagement.yaml"))
}

pub fn load_engagement() -> Result<Engagement> {
    read_yaml(&engagement_path()?)
}

pub fn save_engagement(engagement: &Engagement) -> Result<()> {
    write_yaml(&engagement_path()?, engagement)
}
