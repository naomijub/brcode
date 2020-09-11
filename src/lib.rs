pub mod aux;
pub mod model;
pub mod parse;

pub fn from_str(code: &str) -> Vec<(usize, parse::Data)> {
    parse::parse(code, 99)
}
