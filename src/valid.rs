use crate::constants;

/// Valid version number
/// ```
/// use vern::valid;
/// let version = "0.1.2-re.1.z266.1.2.3+build";
/// let is_version = valid::valid(version);
/// assert!(is_version);
/// ```
pub fn valid(version: &str) -> bool {
    constants::LOOSE_RE.is_match(version)
}
