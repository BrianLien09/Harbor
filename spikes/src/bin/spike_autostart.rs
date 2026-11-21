//! Registers Harbor's logon task through the Task Scheduler API (what
//! `autostart::set` now does) for the given harbor.exe, and reports.
//! `spike_autostart <path to harbor.exe>` or `spike_autostart off`.

fn main() -> anyhow::Result<()> {
    let arg = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("usage: spike_autostart <harbor.exe path | off>"))?;
    let target = (arg != "off").then_some(arg.as_str());
    match harbor_shell::autostart::set(target) {
        Ok(()) => println!("ok; is_enabled = {}", harbor_shell::autostart::is_enabled()),
        Err(error) => return Err(error.into()),
    }
    Ok(())
}
