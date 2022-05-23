use std::fmt::Debug;

use crate::*;
use bitflags::bitflags;

pub type Cursor = Rc<RefCell<Scope>>;

#[derive(Default)]
pub struct Scope {
  pub name: Option<String>,
  pub scope_type: ScopeType,
  pub parent: Option<Rc<RefCell<Self>>>,
  pub children: HashMap<String, Cursor>,
  pub funs: HashMap<String, Rc<RefCell<Function>>>,
  pub vars: HashMap<String, Rc<RefCell<Primitive>>>,
}

impl Debug for Scope {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Scope")
      .field("name", &self.name)
      .field("children", &self.children)
      .field("functions", &self.funs)
      .finish()
  }
}

impl Scope {
  pub fn new(
    name: Option<impl AsRef<str>>,
    parent: Option<Rc<RefCell<Self>>>,
  ) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      name: name.map(|n| n.as_ref().to_owned()),
      parent,
      ..Default::default()
    }))
  }
}

#[derive(Debug)]
pub enum ScopeType {
  None,
  Class,
  Module,
  Block,
}

impl Default for ScopeType {
  fn default() -> Self {
    ScopeType::None
  }
}

pub struct Function {
  pub name: Option<String>,
  pub pos_params: Vec<Param>,
  pub key_params: HashMap<String, Param>,
  pub returns: Primitive,
  pub scope: Rc<RefCell<Scope>>,
}

impl Debug for Function {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Function")
      .field("name", &self.name)
      .field("returns", &self.returns)
      .field("key_params", &self.key_params)
      .finish()
  }
}

#[derive(Debug)]
pub struct Param {
  pub t: Primitive,
  pub default: Option<Value>,
}

#[derive(Debug)]
pub struct Value(Type);

#[derive(Debug)]
enum Type {
  HashMap(Primitive, Primitive),
  Array(Primitive),
  Primitive(Primitive),
}

bitflags! {
  pub struct Primitive: u32 {
    const Nil = 0b00000000;
    const Bool = 0b00000001;
    const Integer = 0b00000010;
    const Float = 0b00000100;
    const String = 0b00001000;
    const Unknown = 0b10000000;
  }
}
