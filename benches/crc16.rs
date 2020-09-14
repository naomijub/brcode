use brcode::crc16_ccitt;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let message = message();
    c.bench_function("crc16", |b| b.iter(|| crc16_ccitt(&message)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn message() -> String {
    "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304"
    .to_string()
}
