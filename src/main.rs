mod abst;
mod exp;
mod op;
mod skin;

fn main() {
  let file =
    std::fs::File::open("./example/sampleScore.tsc").unwrap();
  use exp::scoremap::{Scoremap, ScoremapLoadConfig};
  let config =
    ScoremapLoadConfig::new().ignore_invalid_properties(true);
  let score = Scoremap::from_file(file, config).unwrap();
  println!("{:?}", score);
}
