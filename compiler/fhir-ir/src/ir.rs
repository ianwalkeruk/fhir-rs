use serde::{Deserialize, Serialize};

use crate::GoldenTest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl IrVersion {
    pub const CURRENT: IrVersion = IrVersion {
        major: 0,
        minor: 1,
        patch: 0,
    };

    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl std::fmt::Display for IrVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for IrVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl GoldenTest for IrVersion {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_version_current() {
        assert_eq!(IrVersion::CURRENT.major, 0);
        assert_eq!(IrVersion::CURRENT.minor, 1);
        assert_eq!(IrVersion::CURRENT.patch, 0);
    }

    #[test]
    fn test_ir_version_display() {
        let version = IrVersion::new(1, 2, 3);
        assert_eq!(format!("{}", version), "1.2.3");
    }

    #[test]
    fn test_ir_version_yaml_round_trip() {
        let version = IrVersion::CURRENT;
        let yaml = serde_yaml::to_string(&version).unwrap();
        let parsed: IrVersion = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(version, parsed);
    }
}
