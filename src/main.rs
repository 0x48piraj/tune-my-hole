#[cfg(target_os = "linux")]
fn main() -> anyhow::Result<()> {
    tmhole::core::run()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("Tune My Hole is supported on Linux systems running Pi-hole.");
}
