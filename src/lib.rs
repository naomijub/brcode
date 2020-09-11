pub(crate) mod aux;
pub(crate) mod model;
pub(crate) mod parse;

pub use model::{BrCode, Info, Label, MerchantInfo, Template};
pub use parse::Data;

pub fn from_str(code: &str) -> Vec<(usize, parse::Data)> {
    parse::parse(code, 99)
}

pub fn str_to_brcode(code: &str) -> BrCode {
    let codes = parse::parse(code, 99);
    BrCode::from(codes)
}
