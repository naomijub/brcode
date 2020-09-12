pub(crate) mod aux;
pub(crate) mod emit;
pub(crate) mod model;
pub(crate) mod parse;

pub use model::{BrCode, Info, Label, MerchantInfo, Template};
pub use parse::Data;

pub fn from_str(code: &str) -> Vec<(usize, parse::Data)> {
    parse::parse(code, 99)
}

pub fn to_string(code: Vec<(usize, parse::Data)>) -> String {
    emit::emit(code)
}

pub fn str_to_brcode(code: &str) -> BrCode {
    let codes = parse::parse(code, 99);
    BrCode::from(codes)
}

// FFI
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::str;

fn chars_to_string(pointer: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    str::from_utf8(slice).unwrap().to_string()
}

fn to_c_char(s: String) -> *const c_char {
    let cs = CString::new(s.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    mem::forget(cs);
    ptr
}

// Edn
#[no_mangle]
pub extern "C" fn edn_from_brcode(edn: *const c_char) -> *const c_char {
    let edn_str = chars_to_string(edn);
    let brcode = str_to_brcode(&edn_str);
    to_c_char(edn_rs::to_string(brcode))
}

// Json
#[no_mangle]
pub extern "C" fn json_from_brcode(json: *const c_char) -> *const c_char {
    let json_str = chars_to_string(json);
    let brcode = str_to_brcode(&json_str);
    to_c_char(serde_json::to_string(&brcode).unwrap_or("error".to_string()))
}
