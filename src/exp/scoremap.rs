use regex::Regex;

use super::note::Note;

pub struct ScoremapMetadata {}

pub struct Scoremap {
  metadata: ScoremapMetadata,
  notes: Vec<Note>,
}

const COMMENT: &str = r"^[[:space:]]*(#.*)?$";

#[test]
fn pattern_tests() {
  let comment_reg = Regex::new(COMMENT).unwrap();
  assert!(comment_reg.is_match("# This is a comment. "));
  assert!(comment_reg.is_match("  # Indented!"));
  assert!(comment_reg.is_match(""));
}

impl Scoremap {
  pub fn from_file(file: std::fs::File) -> std::io::Result<()> {
    let comment_reg = Regex::new(COMMENT).unwrap();
    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(file);
    for line in reader.lines() {
      let line = &line?;
      if comment_reg.is_match(line) {
        continue;
      }
    }
    Ok(())
    //Ok(Scoremap {})
  }
}
