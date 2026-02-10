use anyhow::Result;
use std::fs;
use std::io::Write;

use crate::{
    blocklist::load_reference_dir,
    config::Config,
    ftl::load_domain_stats,
    intersect::intersect,
    path::Paths,
    state::RunState,
};

pub fn run(paths: &Paths, config: &Config) -> Result<()> {
    if !paths.ftl_db.exists() {
        anyhow::bail!("Pi-hole FTL database not found at {:?}", paths.ftl_db);
    }

    let stats = load_domain_stats(&paths.ftl_db)?;
    let reference = load_reference_dir(&paths.reference_dir)?;
    let empty_reference = reference.is_empty();

    let selected = intersect(stats, reference);

    let tmp_path = paths.managed_list.with_extension("list.tmp");

    // Write managed blocklist
    {
        let mut file = fs::File::create(&tmp_path)?;
        for d in &selected {
            writeln!(file, "{d}")?;
        }
    }

    // Replace
    fs::rename(&tmp_path, &paths.managed_list)?;

    let state = RunState::from_selection(&selected, empty_reference);
    state.write(&paths.state)?;

    if config.output.auto_reload {
        reload_pihole();
    }

    Ok(())
}

fn reload_pihole() {
    let _ = std::process::Command::new("pihole")
        .arg("reloadlists")
        .status();
}
