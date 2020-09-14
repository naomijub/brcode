use brcode::{brcode_to_string, from_str, str_to_brcode, to_string};
use criterion::{criterion_group, criterion_main, Criterion};

fn brcode_benchmark(c: &mut Criterion) {
    let code = code();
    c.bench_function("using brcode", |b| {
        b.iter(|| brcode_to_string(str_to_brcode(&code)))
    });
}

fn vec_benchmark(c: &mut Criterion) {
    let code = code();
    c.bench_function("using vec", |b| b.iter(|| to_string(from_str(&code))));
}

criterion_group!(benches, brcode_benchmark, vec_benchmark);
criterion_main!(benches);

fn code() -> String {
    "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
    .to_string()
}
