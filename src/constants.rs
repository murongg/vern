use lazy_static::lazy_static;
use regex::Regex;

pub static MAX_LENGTH: i32 = 256;

lazy_static! {
    pub static ref LOOSE_RE: Regex = Regex::new(r"^[v=\s]*([0-9]+)\.([0-9]+)\.([0-9]+)(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA
      //-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$").unwrap();

    pub static ref FULL_RE:Regex=Regex::new(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*
      [a-zA-Z-][a-zA-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$").unwrap();
}
