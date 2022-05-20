use crate::*;

pub fn tokenize(path: impl AsRef<Path>) -> Result<()> {
  let file = std::fs::read_to_string(path.as_ref())?;
  let mut stream = file.chars().peekable();

  let root = Rc::new(RefCell::new(Scope::default()));

  let mut app = TokenTree {
    cursor: root.clone(),
    root,
  };

  while let Some(word) = stream.next_word() {
    if let Some(first_char) = word.chars().nth(0) {
      // comments
      if first_char == '#' {
        stream.to_next_line();
        continue;
      }
    }

    match word.as_str() {
      "class" | "module" => {
        app.add_namespace(&mut stream)?;
      }
      _ => {}
    }
  }

  println!("{:#?}", app);

  Ok(())
}

#[derive(Debug)]
struct TokenTree {
  root: Cursor,
  cursor: Cursor,
}

impl TokenTree {
  fn descend_scope(&mut self, child_name: impl AsRef<str>) -> Result<()> {
    let child_name = child_name.as_ref();
    println!("Descending into {}...", child_name);
    let cursor = self.cursor.clone();
    // println!("State of token tree: {:?}", self.root);
    match cursor.borrow().children.get(child_name) {
      Some(child) => {
        self.cursor = child.clone();
      }
      _ => bail!("Undefined constant {}", child_name),
    }
    Ok(())
  }

  fn descend_scope_create(&mut self, child_name: impl AsRef<str>) -> Result<Cursor> {
    let mut cursor = self.cursor.borrow_mut();
    let child = match cursor.children.get(child_name.as_ref()) {
      Some(child) => child.clone(),
      _ => {
        let child = Scope::new(Some(&child_name), Some(self.cursor.clone()));
        cursor
          .children
          .insert(child_name.as_ref().to_owned(), child.clone());
        child
      }
    };
    Ok(child)
  }

  fn add_namespace(&mut self, stream: &mut Peekable<Chars>) -> Result<()> {
    let name = match stream.next_word() {
      Some(name) => name,
      _ => bail!("Expected name definition"),
    };
    if !name.is_capitalized() {
      bail!("Expected constant definition");
    }

    // drive down into the tree
    let mut name_split: Vec<&str> = name.split("::").collect();
    let last = name_split.pop();
    println!("splits: {:?}", &name_split);
    for cons in name_split {
      self.descend_scope(cons)?;
    }

    self.descend_scope_create(last.unwrap())?;

    Ok(())
  }
}
