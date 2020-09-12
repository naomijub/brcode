use brcode::{edn_from_brcode, json_from_brcode, str_to_brcode};
use criterion::{criterion_group, criterion_main, Criterion};

fn rust_criterion_benchmark(c: &mut Criterion) {
    let code = code();
    c.bench_function("to_brcode", |b| b.iter(|| str_to_brcode(&code)));
}

fn edn_criterion_benchmark(c: &mut Criterion) {
    let code = to_c_char(code());
    c.bench_function("edn_from_brcode", |b| b.iter(|| edn_from_brcode(code)));
}

fn json_criterion_benchmark(c: &mut Criterion) {
    let code = to_c_char(code());
    c.bench_function("json_from_brcode", |b| b.iter(|| json_from_brcode(code)));
}

criterion_group!(
    benches,
    rust_criterion_benchmark,
    edn_criterion_benchmark,
    json_criterion_benchmark
);
criterion_main!(benches);

fn code() -> String {
    "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
    .to_string()
}

// FFI resources
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;

fn to_c_char(s: String) -> *const c_char {
    let cs = CString::new(s.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    mem::forget(cs);
    ptr
}
