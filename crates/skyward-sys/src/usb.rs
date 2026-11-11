//! The functions the USB port offers, which init sets up as
//! `sys.usb.config` asks: adb while the recovery waits, with files over MTP
//! beside it while file transfer is on.

use crate::{prop, set_prop, wait_prop};
use anyhow::{ensure, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

static MTP: AtomicBool = AtomicBool::new(false);

/// What the port offers while nothing else has it.
pub fn resting() -> &'static str {
    if MTP.load(Ordering::Relaxed) {
        "mtp,adb"
    } else {
        "adb"
    }
}

/// Whether MTP goes beside adb while the port rests: from when its function
/// has its descriptors until it goes.
pub fn set_mtp(on: bool) {
    MTP.store(on, Ordering::Relaxed);
}

/// Ask init for `config` and wait until it is in place. Going to "none"
/// also waits for adbd to be gone, so its function is free.
pub fn set(config: &str, timeout: Duration) -> Result<()> {
    set_prop("sys.usb.config", config)?;
    wait_prop("sys.usb.state", config, timeout)?;
    if config == "none" {
        let start = Instant::now();
        while !matches!(prop("init.svc.adbd").as_deref(), None | Some("stopped")) {
            ensure!(start.elapsed() < timeout, "adbd still runs {} ms after the port went to none", timeout.as_millis());
            thread::sleep(Duration::from_millis(10));
        }
    }
    Ok(())
}

/// Hand the port to `config` through "none", which stops the programs
/// serving it and takes every function off; init then sets the port up
/// once the programs behind the new configuration are ready.
pub fn switch(config: &str, timeout: Duration) -> Result<()> {
    switch_with(config, |next| set(next, timeout))
}

fn switch_with(config: &str, mut apply: impl FnMut(&str) -> Result<()>) -> Result<()> {
    let changed = apply("none").and_then(|()| apply(config));
    match changed {
        Err(error) if config != "adb" => {
            let restored = apply("none").and_then(|()| apply("adb"));
            match restored {
                Ok(()) => Err(error.context(format!("USB {config} failed; restored adb"))),
                Err(restore) => Err(error.context(format!("USB {config} failed; restoring adb also failed: {restore:#}"))),
            }
        }
        result => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_mtp_binding_restores_adb_and_keeps_the_original_error() {
        let mut requests = Vec::new();
        let error = switch_with("mtp,adb", |next| {
            requests.push(next.to_string());
            anyhow::ensure!(next != "mtp,adb", "MTP readiness timed out");
            Ok(())
        })
        .unwrap_err();
        assert_eq!(requests, ["none", "mtp,adb", "none", "adb"]);
        assert!(format!("{error:#}").contains("MTP readiness timed out"));
        assert!(format!("{error:#}").contains("restored adb"));
    }

    #[test]
    fn successful_binding_does_not_reset_the_port_again() {
        let mut requests = Vec::new();
        switch_with("mtp,adb", |next| {
            requests.push(next.to_string());
            Ok(())
        })
        .unwrap();
        assert_eq!(requests, ["none", "mtp,adb"]);
    }

    #[test]
    fn failure_unbinding_also_restores_adb() {
        let mut requests = Vec::new();
        let error = switch_with("mtp,adb", |next| {
            requests.push(next.to_string());
            anyhow::ensure!(requests.len() != 1, "unbinding timed out");
            Ok(())
        })
        .unwrap_err();
        assert_eq!(requests, ["none", "none", "adb"]);
        assert!(format!("{error:#}").contains("unbinding timed out"));
    }
}
