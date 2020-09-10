pub mod parse;
pub mod model;
pub mod type;

pub fn from_str(code: &str) -> Vec<(usize, parse::Data)> {
    parse::parse(code, 99)
}
