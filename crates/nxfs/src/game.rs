//! Supported MK8 Deluxe executable builds. IDs come from NSO headers, not filenames.
use anyhow::{Result, bail, ensure};

pub const SDK: &str = "ba520906609c503227871a314a94fb5f6ccbe444";
pub const SUBSDK0: &str = "d9de423efc1512328c3c05378c7cb03173facde3";
pub const MAIN_305: &str = "fe941ed5ba14be5d505698da1bbf4fe700000000";
pub const MAIN_401: &str = "158c0c7399aabd7bf83795c2ef3484c700000000";

pub fn version(main: &str, sdk: &str, subsdk0: &str) -> Result<&'static str> {
    let version = match main {
        MAIN_305 => "3.0.5",
        MAIN_401 => "4.0.1",
        _ => bail!("unsupported MK8 Deluxe main build {main}; use update 3.0.5 or 4.0.1"),
    };
    ensure!(sdk == SDK && subsdk0 == SUBSDK0, "MK8 Deluxe {version}: unsupported SDK module combination; prepare the game again from one update");
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_both_supported_updates() {
        assert_eq!(version(MAIN_305, SDK, SUBSDK0).unwrap(), "3.0.5");
        assert_eq!(version(MAIN_401, SDK, SUBSDK0).unwrap(), "4.0.1");
    }

    #[test]
    fn rejects_unknown_and_mixed_modules() {
        assert!(version("unknown", SDK, SUBSDK0).is_err());
        assert!(version(MAIN_305, "unknown", SUBSDK0).is_err());
        assert!(version(MAIN_305, SDK, "unknown").is_err());
    }
}
