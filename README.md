# BR Code

A crate to parse and emit [BR Code](https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf).

## Usage

```toml
[dependencies]
brcode = "0.1.0"
```

## Example

**Parse String**
```rust
use brcode::{
    from_str,
    parse::Data
};

fn main() {
    let code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";

    assert_eq!(from_str(code), expected());
}

fn expected() -> Vec<(usize, Data)> {
    vec![
        (0, Data::Single("01".to_string())), 
        (4, Data::Single("12345678901234".to_string())), 
        (26, Data::Vector(vec![
            (0, Data::Single("BR.GOV.BCB.PIX".to_string())), 
            (1, Data::Single("123e4567-e12b-12d1-a456-426655440000".to_string()))])), 
        (27, Data::Vector(vec![
            (0, Data::Single("BR.COM.OUTRO".to_string())), 
            (1, Data::Single("0123456789".to_string()))])), 
        (52, Data::Single("0000".to_string())), 
        (53, Data::Single("986".to_string())), 
        (54, Data::Single("123.45".to_string())), 
        (58, Data::Single("BR".to_string())), 
        (59, Data::Single("NOME DO RECEBEDOR".to_string())), 
        (60, Data::Single("BRASILIA".to_string())), 
        (61, Data::Single("70074900".to_string())), 
        (62, Data::Vector(vec![
            (5, Data::Single("RP12345678-2019".to_string()))])), 
        (80, Data::Vector(vec![(
            0, Data::Single("BR.COM.OUTRO".to_string())), 
            (1, Data::Single("0123.ABCD.3456.WXYZ".to_string()))])), 
        (63, Data::Single("AD38".to_string()))]
}
```
## Benchmark

**from_str** in `benches/parse.rs`
```
time:   [16.200 us 16.251 us 16.319 us] 
```

## Goals
- [x] Parse BR Code String;
- [ ] Parse BR Code to `BrCode` struct;
- [ ] Emit BR COde from `BrCode` struct;
- [ ] FFI
    - [ ] Open FFI
    - [ ] Clojure
