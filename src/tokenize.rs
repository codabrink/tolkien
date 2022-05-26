use regex::internal::Char;

use crate::*;

pub fn tokenize(path: impl AsRef<Path>) -> Result<()> {
  let mut reader = FileReader::new(path);

  let root = Rc::new(RefCell::new(Scope::default()));

  let mut app = TokenTree {
    cursor: root.clone(),
    root,
  };
  while let Some(expression) = reader.next_expression() {
    match expression {
      Expression::ClassOpen(name) => {
        app.add_namespace(name);
      }
      Expression::FnDef(name, params) => {
        app.add_function(name);
      }
      Expression::Close => {
        app.close_scope();
      }
      _ => {}
    }
  }

  println!("{:#?}", app.root);

  Ok(())
}

#[derive(Debug)]
pub struct TokenTree {
  root: Cursor,
  cursor: Cursor,
}

impl TokenTree {
  pub const PARAM_DELIM: &'static [char] = &[':', ',', ')'];

  fn close_scope(&mut self) -> Result<()> {
    let cursor = self.cursor.borrow();
    if let Some(parent) = &cursor.parent {}
    Ok(())
  }

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

  fn add_function(&mut self, name: String) -> Result<()> {
    let mut function = Function {
      name: Some(name.clone()),
      scope: self.cursor.clone(),
      returns: Primitive::Unknown,
      key_params: HashMap::new(),
      pos_params: vec![],
    };

    let mut cursor = self.cursor.borrow_mut();
    cursor.funs.insert(name, Rc::new(RefCell::new(function)));

    Ok(())
  }

  fn add_namespace(&mut self, name: String) -> Result<()> {
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
