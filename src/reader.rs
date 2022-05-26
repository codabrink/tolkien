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

pub struct FileReader {
  row: usize,
  col: usize,
  stack: Vec<Nesting>,
  string: String,
  stream: Peekable<Chars<'static>>,
}

enum Nesting {
  String,
  Array,
  HashMap,
  Class,
  Module,
  Function,
}

pub enum Expression {
  // (name)
  ClassOpen(String),
  // (name, param string)
  FnDef(String, Option<String>),
  // (end) - pop off of the stack
  Close,
  Assignment(String, Type),
  Unknown,
}

pub trait TolkienChars {
  fn skip_blank(&mut self);
  fn to_next_line(&mut self);
  fn next_line(&mut self) -> Option<String>;
  fn next_expression(&mut self) -> Option<Expression>;
  fn next_constant_name(&mut self) -> String;
  fn next_function_name(&mut self) -> String;
  fn next_function_parameters(&mut self) -> Option<String>;
  fn next_word(&mut self) -> Option<String>;
  fn next_alphanumeric_word(&mut self) -> Option<String>;
  fn next_word_expected(&mut self) -> String;
  fn read_until(&mut self, delimiters: &[char]) -> (String, Option<char>);
  fn read_until_delim(&mut self, delimiters: &[char]) -> (String, Option<char>);
  fn read_until_delim_inclusive(&mut self, delimiters: &[char]) -> String;
}

impl FileReader {
  pub fn new(path: impl AsRef<Path>) -> Self {
    std::fs::read_to_string(path.as_ref())
      .expect(&format!("Could not open file {:?}", path.as_ref()))
      .into()
  }

  #[inline]
  pub fn peek(&mut self) -> Option<&char> {
    self.stream.peek()
  }

  #[inline]
  pub fn next(&mut self) -> Option<char> {
    self.stream.next()
  }
}

impl From<String> for FileReader {
  fn from(string: String) -> Self {
    let stream = unsafe { transmute::<Chars, Chars<'static>>(string.chars()).peekable() };

    Self {
      row: 0,
      col: 0,
      stack: Vec::new(),
      string,
      stream,
    }
  }
}

impl TolkienChars for FileReader {
  fn next_constant_name(&mut self) -> String {
    let name = self.next_word().expect("Expected constant definition.");
    if !name.is_capitalized() {
      panic!("{} is not a constant, expected constant.", name);
    }
    name
  }

  fn next_function_name(&mut self) -> String {
    let (name, delim) = self.read_until_delim(&['(']);

    if name.is_blank() {
      panic!("Expected function name.");
    }
    name
  }

  fn next_function_parameters(&mut self) -> Option<String> {
    let params = self.read_until_delim_inclusive(&[')']);
    if params.is_blank() {
      None
    } else {
      Some(params)
    }
  }

  fn next_expression(&mut self) -> Option<Expression> {
    self.skip_blank();
    let word = self.next_word()?;

    match word.as_str() {
      "class" | "module" => {
        self.stack.push(Nesting::Class);
        return Some(Expression::ClassOpen(self.next_constant_name()));
      }
      "def" => {
        self.stack.push(Nesting::Function);
        return Some(Expression::FnDef(
          self.next_function_name(),
          self.next_function_parameters(),
        ));
      }
      "end" => {
        if let None = self.stack.pop() {
          panic!("Unexpected keyword \"end\".")
        }
        return Some(Expression::Close);
      }
      _ => return Some(Expression::Unknown),
    }
  }

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
    self.skip_blank();
    while let Some(char) = self.peek() {
      if delimiters.contains(char) {
        return (result, Some(*char));
      }
      result.push(self.next().unwrap());
    }
    (result, None)
  }

  fn read_until_delim(&mut self, delimiters: &[char]) -> (String, Option<char>) {
    let mut result = String::new();
    self.skip_blank();

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

  fn read_until_delim_inclusive(&mut self, delimiters: &[char]) -> String {
    let (mut result, delim) = self.read_until_delim(delimiters);
    if let Some(delim) = delim {
      // discard the char in the iter
      self.stream.next();

      result.push(delim);
    }
    result
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
  fn next_line(&mut self) -> Option<String> {
    let mut result = String::new();
    let mut has_value = false;

    while let Some(c) = self.next() {
      if c == '\n' || c == ';' {
        return Some(result);
      }
      result.push(c);
      has_value = true
    }

    if has_value {
      Some(result)
    } else {
      None
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
  fn reader() -> Result<()> {
    let mut stream = FileReader::new(Path::new("assets/test.rb"));
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
  fn is_blank(&self) -> bool;
}
impl TolkienString for String {
  #[inline]
  fn is_capitalized(&self) -> bool {
    if let Some(c) = self.chars().nth(0) {
      return c.is_ascii_uppercase();
    }
    false
  }
  #[inline]
  fn is_blank(&self) -> bool {
    self.len() == 0
  }
}
