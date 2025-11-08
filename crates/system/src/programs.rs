use std::path::PathBuf;

use anyhow::{Result, bail};
use log::{error, info};

const SEARCH_DIRECTORY: &str = "tools/bin";
const REQUIRED_PROGRAMS: &[(&str, &[&str; 2])] = &[
    ("tex3ds", &["tex3ds", "mkbcfnt"]),
    ("3dstools", &["3dsxtool", "smdhtool"]),
    ("switch-tools", &["nacptool", "elf2nro"]),
    ("wut-tools", &["elf2rpl", "wuhbtool"]),
];

pub fn get_binary(binary: &str) -> PathBuf {
    match std::env::var("DEVKITPRO") {
        Ok(value) => PathBuf::from(value).join(SEARCH_DIRECTORY).join(binary),
        Err(_) => PathBuf::from(binary),
    }
}

fn check_binary(binary: &str) -> bool {
    let name = get_binary(binary);
    if which::which(name).is_err() {
        error!("✘ {binary} is not installed or in PATH.");
        return false;
    }
    info!("✓ Found binary {binary}");
    true
}

pub fn check_environment() -> Result<()> {
    info!("Starting environment check for required programs...");
    let mut missing = Vec::new();

    for &(group, binaries) in REQUIRED_PROGRAMS {
        info!("Checking group '{group}'");
        for binary in binaries {
            if !check_binary(binary) {
                missing.push(binary);
            }
        }
    }

    if !missing.is_empty() {
        bail!("Required programs are missing: {missing:?}");
    }

    info!("All required programs are installed and environment is ready.");
    Ok(())
}
