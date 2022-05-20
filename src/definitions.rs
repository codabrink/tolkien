use bitflags::bitflags;
use hashbrown::HashMap;

#[derive(Default)]
pub struct Namespace {
  name: String,
  children: Vec<Self>,
  funs: Vec<Function>,
  vars: Vec<Type>,
}

pub struct Function {
  pos_params: Vec<Param>,
  key_param: HashMap<String, Param>,
  params: Vec<Param>,
  returns: Type,
}

pub struct Param {
  t: Type,
  default: Option<Value>,
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
