pub(crate) mod aux;
pub(crate) mod emit;
pub(crate) mod model;
pub(crate) mod parse;

pub use aux::crc16_ccitt;
pub use model::{BrCode, Info, Label, MerchantInfo, Template};
pub use parse::Data;
pub use qrcode_generator::QrCodeEcc;

pub fn from_str(code: &str) -> Vec<(usize, parse::Data)> {
    parse::parse(code, 99)
}

pub fn to_string(code: &[(usize, parse::Data)]) -> String {
    emit::emit(&code)
}

pub fn brcode_to_string(code: BrCode) -> String {
    code.encode()
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

fn to_c_char(s: &str) -> *const c_char {
    let cs = CString::new(s.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    mem::forget(cs);
    ptr
}

#[no_mangle]
pub extern "C" fn crc16_ccitt_from_message(message: *const c_char) -> *const c_char {
    let message_str = chars_to_string(message);
    let checksum = crc16_ccitt(&message_str);
    to_c_char(&checksum)
}

// Edn
#[no_mangle]
pub extern "C" fn edn_from_brcode(edn: *const c_char) -> *const c_char {
    let edn_str = chars_to_string(edn);
    let brcode = str_to_brcode(&edn_str);
    to_c_char(&edn_rs::to_string(brcode))
}

#[no_mangle]
pub extern "C" fn edn_to_brcode(edn: *const c_char) -> *const c_char {
    let edn_str = chars_to_string(edn);
    let brcode: BrCode = edn_rs::from_str(&edn_str).unwrap();

    to_c_char(&brcode_to_string(brcode))
}

#[no_mangle]
pub extern "C" fn edn_to_svg_brcode(edn: *const c_char) -> *const c_char {
    let edn_str = chars_to_string(edn);
    let brcode: BrCode = edn_rs::from_str(&edn_str).unwrap();
    let svg = brcode.to_svg_standard_string();

    to_c_char(&svg)
}

#[no_mangle]
pub extern "C" fn edn_to_svg_file(edn: *const c_char, file_path: *const c_char) {
    let edn_str = chars_to_string(edn);
    let file_path_str = chars_to_string(file_path);
    let brcode: BrCode = edn_rs::from_str(&edn_str).unwrap();
    brcode.to_standard_svg_file(&file_path_str);
}

// Json
#[no_mangle]
pub extern "C" fn json_from_brcode(json: *const c_char) -> *const c_char {
    let json_str = chars_to_string(json);
    let brcode = str_to_brcode(&json_str);
    to_c_char(&serde_json::to_string(&brcode).unwrap_or_else(|_| "error".to_string()))
}

#[no_mangle]
pub extern "C" fn json_to_brcode(json: *const c_char) -> *const c_char {
    let json_str = chars_to_string(json);
    let brcode: BrCode = serde_json::from_str(&json_str).unwrap();

    to_c_char(&brcode_to_string(brcode))
}

#[no_mangle]
pub extern "C" fn json_to_svg_brcode(json: *const c_char) -> *const c_char {
    let json_str = chars_to_string(json);
    let brcode: BrCode = serde_json::from_str(&json_str).unwrap();
    let svg = brcode.to_svg_standard_string();

    to_c_char(&svg)
}

#[no_mangle]
pub extern "C" fn json_to_svg_file(json: *const c_char, file_path: *const c_char) {
    let json_str = chars_to_string(json);
    let file_path_str = chars_to_string(file_path);
    let brcode: BrCode = serde_json::from_str(&json_str).unwrap();
    brcode.to_standard_svg_file(&file_path_str);
}
