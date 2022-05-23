use crate::*;

pub trait TolkienChar {
  fn blank(&self) -> bool;
}
impl TolkienChar for char {
  #[inline]
  fn blank(&self) -> bool {
    *self == ' ' || *self == '\n'
  }
}

struct FileReader<'a> {
  line: usize,
  index: usize,
  stream: Peekable<Chars<'a>>,
}

pub trait TolkienChars {
  fn skip_blank(&mut self);
  fn to_next_line(&mut self);
  fn next_word(&mut self) -> Option<String>;
  fn next_alphanumeric_word(&mut self) -> Option<String>;
  fn next_word_expected(&mut self) -> String;
  fn read_until(&mut self, delimiters: &[char]) -> (String, Option<char>);
  fn read_until_blank_or(&mut self, delimiters: &[char]) -> (String, Option<char>);
}

impl<'a> TolkienChars for Peekable<Chars<'a>> {
  #[inline]
  fn skip_blank(&mut self) {
    while let Some(char) = self.peek() {
      if !char.blank() {
        break;
      }
      let _ = self.next();
    }
  }

  // (result, delimiting char)
  fn read_until(&mut self, delimiters: &[char]) -> (String, Option<char>) {
    let mut result = String::new();
    while let Some(char) = self.peek() {
      if delimiters.contains(char) {
        return (result, Some(*char));
      }
      result.push(self.next().unwrap());
    }
    (result, None)
  }

  fn read_until_blank_or(&mut self, delimiters: &[char]) -> (String, Option<char>) {
    let mut result = String::new();

    while let Some(char) = self.peek() {
      if char.blank() {
        break;
      }
      if delimiters.contains(char) {
        return (result, Some(*char));
      }
      result.push(self.next().unwrap());
    }

    (result, None)
  }

  #[inline]
  fn to_next_line(&mut self) {
    while let Some(char) = self.peek() {
      if *char == '\n' {
        break;
      }
      let _ = self.next();
    }
  }

  #[inline]
  fn next_word(&mut self) -> Option<String> {
    self.skip_blank();
    let mut word = String::new();

    if self.peek().is_none() {
      return None;
    }

    while let Some(c) = self.peek() {
      if c.blank() {
        break;
      }
      word.push(self.next().unwrap());
    }

    Some(word)
  }

  #[inline]
  fn next_alphanumeric_word(&mut self) -> Option<String> {
    self.skip_blank();
    let mut word = String::new();

    if self.peek().is_none() {
      return None;
    }

    while let Some(c) = self.peek() {
      if !c.is_alphanumeric() && *c != '_' {
        break;
      }
      word.push(self.next().unwrap());
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

pub trait TolkienOptString {
  fn present(&self) -> bool;
  fn is_capitalized(&self) -> bool;
}
impl TolkienOptString for Option<String> {
  #[inline]
  fn present(&self) -> bool {
    if let Some(s) = self {
      return s.len() != 0;
    }
    false
  }

  #[inline]
  fn is_capitalized(&self) -> bool {
    if let Some(s) = self {
      return s.is_capitalized();
    }
    false
  }
}
pub trait TolkienString {
  fn is_capitalized(&self) -> bool;
}
impl TolkienString for String {
  #[inline]
  fn is_capitalized(&self) -> bool {
    if let Some(c) = self.chars().nth(0) {
      return c.is_ascii_uppercase();
    }
    false
  }
}
