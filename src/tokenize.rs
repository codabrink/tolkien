use crate::*;

pub fn tokenize(path: impl AsRef<Path>) -> Result<()> {
  let file = std::fs::read_to_string(path.as_ref())?;
  let mut stream = file.chars().peekable();

  let mut root = Namespace {
    name: "root".to_owned(),
    ..Default::default()
  };

  if let Some(word) = stream.next_word() {
    match word.as_str() {
      "class" => {
        namespace(&mut stream);
      }
      _ => {}
    }
  }

  Ok(())
}

fn namespace(stream: &mut Peekable<Chars>) -> Result<Namespace> {
  let name = stream.next_word();
  if name.is_none() || name.as_ref().unwrap().len() == 0 {
    bail!("Expected name definition");
  }
  if let Some(name) = name.as_ref() {
    if !name.chars().nth(0).unwrap().is_ascii_uppercase() {
      bail!("Expected constant definition: {}", name);
    }
  }

  Ok(Namespace::default())
}
