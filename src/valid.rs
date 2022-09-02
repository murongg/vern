use crate::constants;

pub fn valid(version: &str) -> bool{
  constants::LOOSE_RE.is_match(version)
}
