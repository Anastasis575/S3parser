use anyhow::Result;

mod episode_data;
mod episode_md;
fn main() -> Result<()> {
    // episode_md::parse_episode_to_md();
    episode_data::parse_episode()
}
