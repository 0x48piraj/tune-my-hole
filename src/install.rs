use anyhow::Result;
use std::fs;
use std::process::Command;

use crate::{config::Config, path::Paths};

pub fn init_system(paths: &Paths) -> Result<()> {
    fs::create_dir_all(&paths.reference_dir)?;

    for p in [
        &paths.managed_list,
        &paths.meta,
        &paths.state,
    ] {
        if !p.exists() {
            fs::write(p, "")?;
        }
    }

    Config::write_default(&paths.config).ok();

    if !install_systemd() {
        install_cron();
    }

    Ok(())
}

fn install_systemd() -> bool {
    let service = r#"[Unit]
Description=Tune My Hole adaptive blocklist

[Service]
Type=oneshot
ExecStart=/usr/local/bin/tune-my-hole run
Nice=10
IOSchedulingClass=idle
"#;

    let timer = r#"[Unit]
Description=Run Tune My Hole daily

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
"#;

    if fs::write("/etc/systemd/system/tune-my-hole.service", service).is_err() {
        return false;
    }

    if fs::write("/etc/systemd/system/tune-my-hole.timer", timer).is_err() {
        return false;
    }

    let _ = Command::new("systemctl").args(["daemon-reload"]).status();
    let _ = Command::new("systemctl")
        .args(["enable", "--now", "tune-my-hole.timer"])
        .status();

    true
}

fn install_cron() {
    let entry = "0 3 * * * /usr/local/bin/tune-my-hole run";

    let cmd = format!(
        "(crontab -l 2>/dev/null | grep -v tune-my-hole; echo \"{}\") | crontab -",
        entry
    );

    let _ = Command::new("sh").arg("-c").arg(cmd).status();
}

pub fn uninstall_system(paths: &Paths) -> Result<()> {
    uninstall_systemd();
    uninstall_cron();

    for p in [
        &paths.managed_list,
        &paths.meta,
        &paths.state,
        &paths.config,
    ] {
        let _ = std::fs::remove_file(p);
    }

    let _ = std::fs::remove_dir(&paths.reference_dir);

    Ok(())
}

fn uninstall_systemd() {
    let _ = Command::new("systemctl")
        .args(["disable", "--now", "tune-my-hole.timer"])
        .status();

    let _ = std::fs::remove_file("/etc/systemd/system/tune-my-hole.timer");
    let _ = std::fs::remove_file("/etc/systemd/system/tune-my-hole.service");

    let _ = Command::new("systemctl")
        .args(["daemon-reload"])
        .status();
}

fn uninstall_cron() {
    let cmd = "(crontab -l 2>/dev/null | grep -v tune-my-hole) | crontab -";
    let _ = Command::new("sh").arg("-c").arg(cmd).status();
}
