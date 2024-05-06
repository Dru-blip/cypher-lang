use std::fmt::Display;

#[derive(Debug,Clone)]
pub enum Object{
    Number(f64),
    Nil,
    Boolean(bool),
    Str(String)
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(a) => write!(f, "{}", a),
            Object::Nil => write!(f,"{}",self),
            Object::Boolean(b) => write!(f,"{}",b),
            Object::Str(s) => write!(f,"{}",s),
        }
    }
}