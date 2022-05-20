#[allow(non_upper_case_globals)]
// classes
mod definitions;
mod prelude;
mod reader;
mod tokenize;

use definitions::*;
use prelude::*;
use reader::*;
use std::path::Path;

fn main() {
  tokenize::tokenize(Path::new("assets/test.rb"));
}
