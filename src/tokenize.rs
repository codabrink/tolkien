use anyhow::{bail, Result};
use std::{path::Path, str::Chars};

use crate::definitions::Namespace;

fn tokenize(path: impl AsRef<Path>) -> Result<()> {
  let file = std::fs::read_to_string(path.as_ref())?;

  Ok(())
}

// fn class(chars: &mut Chars) -> Result<Nampespace> {}

trait TolkeinChar {
  fn blank(&self) -> bool;
}
impl TolkeinChar for char {
  #[inline]
  fn blank(&self) -> bool {
    *self == ' ' || *self == '\n'
  }
}

trait TolkeinChars {
  fn skip_blank(&mut self);
  fn next_word(&mut self) -> Option<String>;
}

impl<'a> TolkeinChars for Chars<'a> {
  #[inline]
  fn skip_blank(&mut self) {
    self.skip_while(|c| c.blank());
  }

  #[inline]
  fn next_word(&mut self) -> Option<String> {
    self.skip_blank();
    let mut word = String::new();
    for c in self {
      if c.blank() {
        break;
      }
      word.push(c);
    }
    Some(word)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_stuff() {
    let a = String::with_capacity(4);
    println!("{}", a.capacity());
  }
}
