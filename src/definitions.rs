use crate::*;
use bitflags::bitflags;

#[derive(Default)]
pub struct Namespace {
  pub name: String,
  pub open: bool,
  pub parent: Rc<RefCell<Self>>,
  pub children: HashMap<String, Rc<RefCell<Self>>>,
  pub funs: Vec<Rc<RefCell<Function>>>,
  pub vars: Vec<Rc<RefCell<Type>>>,
}

impl Namespace {}

pub struct Function {
  pub pos_params: Vec<Param>,
  pub key_param: HashMap<String, Param>,
  pub returns: Type,
}

pub struct Param {
  pub t: Type,
  pub default: Option<Value>,
}

pub struct Value(Type);

bitflags! {
  pub struct Type: u32 {
    const Nil = 0b00000000;
    const Bool = 0b00000001;
    const Integer = 0b00000010;
    const Float = 0b00000100;
    const String = 0b00001000;
    const Unknown = 0b10000000;
  }
}

pub fn test() {
  let a = Type::Nil | Type::String;
  assert_eq!(a & Type::String, Type::String);
}

trait Tokenize {
  const DELIMITER: &'static str;
}

impl Tokenize for Namespace {
  const DELIMITER: &'static str = " \n";
}
