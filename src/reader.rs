use crate::*;

pub trait TolkeinChar {
  fn blank(&self) -> bool;
}
impl TolkeinChar for char {
  #[inline]
  fn blank(&self) -> bool {
    *self == ' ' || *self == '\n'
  }
}

struct TokenStream {
  line: usize,
  index: usize,
  stream: Peekable<Chars>,
}

pub trait TolkeinChars {
  fn skip_blank(&mut self);
  fn next_word(&mut self) -> Option<String>;
  fn next_word_expected(&mut self) -> String;
}

impl<'a> TolkeinChars for Peekable<Chars<'a>> {
  #[inline]
  fn skip_blank(&mut self) {
    while let Some(char) = self.peek() {
      if !char.blank() {
        break;
      }
      let _ = self.next();
    }
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

  fn next_word_expected(&mut self) -> String {
    self.next_word().expect("Unexpected eol")
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn skip_blank() {
    let mut stream = " \nHello there".chars().peekable();
    stream.skip_blank();

    let hello_there: String = stream.collect();
    assert_eq!("Hello there", hello_there.as_str());
  }

  #[test]
  fn reader() -> Result<()> {
    let file = std::fs::read_to_string(Path::new("assets/test.rb"))?;
    let mut stream = file.chars().peekable();
    let word = stream.next_word().unwrap();

    assert_eq!("class", word.as_str());

    Ok(())
  }
}
