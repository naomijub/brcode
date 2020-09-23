# BR Code

A crate to parse and emit [PIX BR Code](https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf).
* [Technical and Business specs for BR Code usage](https://www.bcb.gov.br/content/estabilidadefinanceira/forumpireunioes/Anexo%20I%20-%20QRCodes%20-%20Especifica%C3%A7%C3%A3o%20-%20vers%C3%A3o%201-1.pdf)

## Important Changes

* Version `1.2` has a small break for a `BrCode` field. [PR](https://github.com/naomijub/brcode/pull/14) fixes `model::BrCode` field `initiation_method` naming.

## Usage

```toml
[dependencies]
brcode = "1.3.1"
```

### Build from source
1. Install [rustup](https://rustup.rs/).
2. `make build-macos` for macos and ios files or `make build-linux` for linux and android files.
3. Files will be located at `target/release/libbrcode.*`, `target/<target-platform>/release/libbrcode.so`.


### Copy files from Github Release
Shellscript to get files from release:

**So**
```sh
curl -s https://api.github.com/repos/naomijub/brcode/releases/latest \
| grep "browser_download_url.*so" \
| cut -d : -f 2,3 \
| tr -d \" \
| wget -qi -
```

**dylib**
```sh
curl -s https://api.github.com/repos/naomijub/brcode/releases/latest \
| grep "browser_download_url.*dylib" \
| cut -d : -f 2,3 \
| tr -d \" \
| wget -qi -
```

## Example

**Parse String**
```rust
use brcode::{from_str, Data};

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

**str_to_brcode**
```rust
use brcode::{str_to_brcode, BrCode, Template, Info, MerchantInfo, Label};


fn main() {
    let code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";

    assert_eq!(from_str(code), expected());
}

fn expected() -> BrCode {
    BrCode {
        payload_version: 1,
        initiation_method: None,
        merchant_category_code: 0000u32,
        merchant_name: "NOME DO RECEBEDOR".to_string(),
        merchant_city: "BRASILIA".to_string(),
        merchant_information: vec![
            MerchantInfo {
                id: 26,
                info: vec![
                    Info {
                        id: 0,
                        info: "BR.GOV.BCB.PIX".to_string(),
                    },
                    Info {
                        id: 1,
                        info: "123e4567-e12b-12d1-a456-426655440000".to_string(),
                    },
                ],
            },
            MerchantInfo {
                id: 27,
                info: vec![
                    Info {
                        id: 0,
                        info: "BR.COM.OUTRO".to_string(),
                    },
                    Info {
                        id: 1,
                        info: "0123456789".to_string(),
                    },
                ],
            },
        ],
        currency: "986".to_string(),
        postal_code: Some("70074900".to_string()),
        amount: Some(123.45),
        country_code: "BR".to_string(),
        field_template: vec![Label {
            reference_label: "RP12345678-2019".to_string(),
        }],
        crc1610: "AD38".to_string(),
        templates: Some(vec![Template {
            id: 80usize,
            info: vec![
                Info {
                    id: 0usize,
                    info: "BR.COM.OUTRO".to_string(),
                },
                Info {
                    id: 1usize,
                    info: "0123.ABCD.3456.WXYZ".to_string(),
                },
            ],
        }]),
    }
}
```

**brcode::to_string** from `Vec<(usize, Data)>`:
```rust
use brcode::{
    self, BrCode, 
};

fn main() {
    let actual = brcode::to_string(brcode_vec());
    let code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";

    assert_eq!(actual, code);
}

fn brcode_vec() -> Vec<(usize, Data)> {
    vec![
        (0, Data::Single("01".to_string())),
        (4, Data::Single("12345678901234".to_string())),
        (
            26,
            Data::Vector(vec![
                (0, Data::Single("BR.GOV.BCB.PIX".to_string())),
                (
                    1,
                    Data::Single("123e4567-e12b-12d1-a456-426655440000".to_string()),
                ),
            ]),
        ),
        (
            27,
            Data::Vector(vec![
                (0, Data::Single("BR.COM.OUTRO".to_string())),
                (1, Data::Single("0123456789".to_string())),
            ]),
        ),
        (52, Data::Single("0000".to_string())),
        (53, Data::Single("986".to_string())),
        (54, Data::Single("123.45".to_string())),
        (58, Data::Single("BR".to_string())),
        (59, Data::Single("NOME DO RECEBEDOR".to_string())),
        (60, Data::Single("BRASILIA".to_string())),
        (61, Data::Single("70074900".to_string())),
        (
            62,
            Data::Vector(vec![(5, Data::Single("RP12345678-2019".to_string()))]),
        ),
        (
            80,
            Data::Vector(vec![
                (0, Data::Single("BR.COM.OUTRO".to_string())),
                (1, Data::Single("0123.ABCD.3456.WXYZ".to_string())),
            ]),
        ),
        (63, Data::Single("AD38".to_string())),
    ]
}

```

**brcode::brcode_to_string** for struct `BrCode`:
```rust
use brcode::{
    self, BrCode, 
};

fn main() {
    let actual = brcode::brcode_to_string(brcode_value());
    let code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";

    assert_eq!(actual, code);
}

fn brcode_value() -> BrCode {
    BrCode {
        payload_version: 1,
        initiation_method: None,
        merchant_account_information: Some(String::from("12345678901234")),
        merchant_category_code: 0000u32,
        merchant_name: "NOME DO RECEBEDOR".to_string(),
        merchant_city: "BRASILIA".to_string(),
        merchant_information: vec![
            MerchantInfo {
                id: 26,
                info: vec![
                    Info {
                        id: 0,
                        info: "BR.GOV.BCB.PIX".to_string(),
                    },
                    Info {
                        id: 1,
                        info: "123e4567-e12b-12d1-a456-426655440000".to_string(),
                    },
                ],
            },
            MerchantInfo {
                id: 27,
                info: vec![
                    Info {
                        id: 0,
                        info: "BR.COM.OUTRO".to_string(),
                    },
                    Info {
                        id: 1,
                        info: "0123456789".to_string(),
                    },
                ],
            },
        ],
        currency: "986".to_string(),
        postal_code: Some("70074900".to_string()),
        amount: Some(123.45),
        country_code: "BR".to_string(),
        field_template: vec![Label {
            reference_label: "RP12345678-2019".to_string(),
        }],
        crc1610: "AD38".to_string(),
        templates: Some(vec![Template {
            id: 80usize,
            info: vec![
                Info {
                    id: 0usize,
                    info: "BR.COM.OUTRO".to_string(),
                },
                Info {
                    id: 1usize,
                    info: "0123.ABCD.3456.WXYZ".to_string(),
                },
            ],
        }]),
    }
}
```

**`BrCode` functions**
* `pub fn is_pix(&self) -> bool` determines if BrCode is a `PIX` transaction.
* `pub fn get_transaction_id(&self) -> Option<String>` gets `transaction_id` value (field 5 of item 65).
* `pub fn get_alias(&self) -> Option<Vec<String>>` gets all possible values for PIX aliases. Usually only field 1 of item 26 is valid. Checks if the alias if of type `"BR.GOV.BCB.PIX"`.
* `pub fn get_message(&self) -> Option<Vec<String>>` gets all possible massages for PIX aliases. Usually only field 2 of item 26 is valid. Checks if the alias if of type `"BR.GOV.BCB.PIX"`.

## Benchmark

**from_str** in `benches/parse.rs`
```
time:   [15.734 us 15.758 us 15.782 us]
```

**str_to_brcode** in `benches/to_brcode`
```
time:   [24.886 us 24.931 us 24.977 us]
```

**edn_from_brcode** in `benches/to_brcode`
```
time:   [52.670 us 52.795 us 52.929 us]
```

**json_from_brcode** in `benches/to_brcode`
```
time:    [28.229 us 28.284 us 28.339 us]
```

**both-ways using `BrCode`**
```
time:   [33.238 us 33.555 us 33.924 us]                          
```

**both-ways using `Vec<(usize, Data)>`**
```
time:   [22.867 us 22.958 us 23.107 us]
```

**crc16_ccitt** in `benches/crc16`:
```
time:   [3.0738 us 3.0825 us 3.0938 us]
```

## FFI

### Clojure FFI 
[DOCS](https://github.com/naomijub/brcode/blob/master/clj-brcode/README.md)

**BR Code as Edn** call function FFI `edn_from_brcode` or use clojar `[clj-brcode "1.1.0-SNAPSHOT"]`. Example:
```clojure
(ns example.core
  (:require [clj-brcode.core :refer :all]))

(def code "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38")

(brcode->edn code)

; {:payload-version 1, :initiation-method nil, :merchant-information [{:id 26, :info [{:id 0, :info "BR.GOV.BCB.PIX"}, {:id 1, :info "123e4567-e12b-12d1-a456-426655440000"}]}, {:id 27, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123456789"}]}], :merchant-category-code 0, :merchant-name "NOME DO RECEBEDOR", :merchant-city "BRASILIA", :postal-code "70074900", :currency "986", :amount 123.45, :country-code "BR", :field-template [{:reference-label "RP12345678-2019"}], :crc1610 "AD38", :templates [{:id 80, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123.ABCD.3456.WXYZ"}]}]}
```

Input:
```rust
"00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```

Expected Edn:
```clojure
{:payload-version 1,:initiation-method nil, :merchant-information [
  {:id 26, :info [{ :id 0, :info "BR.GOV.BCB.PIX",}, {:id 1, :info "123e4567-e12b-12d1-a456-426655440000",}]},
  {:id 27, :info [{ :id 0, :info "BR.COM.OUTRO",}, {:id 1, :info "0123456789",}]}
 ],:merchant-category-code 0, :merchant-name "NOME DO RECEBEDOR", :merchant-city "BRASILIA", 
 :postal-code "70074900", :currency "986", :amount 123.45, :country-code "BR", 
 :field-template [{ :reference-label "RP12345678-2019", }], :crc1610 "AD38", :templates [
   { :id 80, :info [{ :id 0, :info "BR.COM.OUTRO", },{ :id 1, :info "0123.ABCD.3456.WXYZ", }], }]
 }
```


**Edn as BR Code** call function FFI `edn_to_brcode` or use clojar `[clj-brcode "1.1.0-SNAPSHOT"]`. Example:
```clojure
(ns example.core
  (:require [clj-brcode.core :refer :all]))

(def edn {:payload-version 1, :initiation-method nil, :merchant-information [{:id 26, :info [{:id 0, :info "BR.GOV.BCB.PIX"}, {:id 1, :info "123e4567-e12b-12d1-a456-426655440000"}]}, {:id 27, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123456789"}]}], :merchant-category-code 0, :merchant-name "NOME DO RECEBEDOR", :merchant-city "BRASILIA", :postal-code "70074900", :currency "986", :amount 123.45, :country-code "BR", :field-template [{:reference-label "RP12345678-2019"}], :crc1610 "AD38", :templates [{:id 80, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123.ABCD.3456.WXYZ"}]}]})

(brcode->edn edn)

; "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```

**Other available functions**:
- `json->brcode`
- `brcode->json`
- `crc16-ccitt`

### Clojure Benchmark with Criterium

**brcode->edn**
```
Evaluation count : 4644 in 6 samples of 774 calls.
             Execution time mean : 131.416626 µs
    Execution time std-deviation : 2.218919 µs
   Execution time lower quantile : 130.073353 µs ( 2.5%)
   Execution time upper quantile : 135.212868 µs (97.5%)
                   Overhead used : 8.079635 ns
```

**edn->brcode**
```
Evaluation count : 3816 in 6 samples of 636 calls.
             Execution time mean : 157.407924 µs
    Execution time std-deviation : 3.556917 µs
   Execution time lower quantile : 154.338082 µs ( 2.5%)
   Execution time upper quantile : 162.800564 µs (97.5%)
                   Overhead used : 8.102766 ns
```

**(-> brcode brcode->edn edn->brcode)**
```
Evaluation count : 1920 in 6 samples of 320 calls.
             Execution time mean : 344.903181 µs
    Execution time std-deviation : 26.518055 µs
   Execution time lower quantile : 328.923528 µs ( 2.5%)
   Execution time upper quantile : 390.059255 µs (97.5%)
                   Overhead used : 8.071450 ns
```

### Node FFI 
[DOCS](https://github.com/naomijub/brcode/blob/master/node-brcode/README.md)

**BR Code as Json** call function `parse`. Example:
```js
const brcode = require('node-brecode');

const code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";

console.log(brcode.parse(code));
// {"payload_version":1,"initiation_method":null, 
// "merchant_information":[
//   {"id":26,"info":[{"id":0,"info":"BR.GOV.BCB.PIX"},{"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}]},
//   {"id":27,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123456789"}]}
// ],
// "merchant_category_code":0,"merchant_name":"NOME DO RECEBEDOR","merchant_city":"BRASILIA","postal_code":"70074900",
// "currency":"986","amount":123.45,"country_code":"BR","field_template":[{"reference_label":"RP12345678-2019"}],
// "crc1610":"AD38","templates":[
//   {"id":80,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123.ABCD.3456.WXYZ"}]}
// ]}
```

Input:
```rust
"00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```

Expected Json:
```json
{"payload_version":1,"initiation_method":null, 
"merchant_information":[
  {"id":26,"info":[{"id":0,"info":"BR.GOV.BCB.PIX"},{"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}]},
  {"id":27,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123456789"}]}
],
"merchant_category_code":0,"merchant_name":"NOME DO RECEBEDOR","merchant_city":"BRASILIA","postal_code":"70074900",
"currency":"986","amount":123.45,"country_code":"BR","field_template":[{"reference_label":"RP12345678-2019"}],
"crc1610":"AD38","templates":[
  {"id":80,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123.ABCD.3456.WXYZ"}]}
]}
```

**Json as BR Code** call function `emit`. Example:
```js
const brcode = require('node-brcode');
const json = {"payload_version":1,"initiation_method":null, 
"merchant_information":[
  {"id":26,"info":[{"id":0,"info":"BR.GOV.BCB.PIX"},{"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}]},
  {"id":27,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123456789"}]}
],
"merchant_category_code":0,"merchant_name":"NOME DO RECEBEDOR","merchant_city":"BRASILIA","postal_code":"70074900",
"currency":"986","amount":123.45,"country_code":"BR","field_template":[{"reference_label":"RP12345678-2019"}],
"crc1610":"AD38","templates":[
  {"id":80,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123.ABCD.3456.WXYZ"}]}
]};

console.log(brcode.emit(json))
// "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```

**Other available functions**:
- `crc16Ccitt`

### Node Benchmark with microbench

**parse**
```js
{ source: 'function() { return parse(code, 1000); }',
    raw: [ 0, 3202224 ],
    duration: '3 ms 202 μs 224 ns',
    name: 'parser' } 
```

**emit**
```js
{ source: 'function() { return emit(json, 1000); }',
    raw: [ 0, 3386206 ],
    duration: '3 ms 386 μs 206 ns',
    name: 'emitter' }
```

**parse(emit(json))**
```js
{ source: 'function() { return parse(emit(json), 1000); }',
    raw: [ 0, 4309501 ],
    duration: '4 ms 309 μs 501 ns',
    name: 'both-ways' } 
```

### Dart FFI
[DOCS](https://github.com/naomijub/brcode/blob/master/dartbrcode/README.md)

**Parse**
```dart
import 'package:dartbrcode/dartbrcode.dart';

final json = '{"payload_version":1,"initiation_method":null,"merchant_account_information":"12345678901234","merchant_information":[{"id":26,"info":[{"id":0,"info":"BR.GOV.BCB.PIX"},{"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}]},{"id":27,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123456789"}]}],"merchant_category_code":0,"merchant_name":"NOME DO RECEBEDOR","merchant_city":"BRASILIA","postal_code":"70074900","currency":"986","amount":123.45,"country_code":"BR","field_template":[{"reference_label":"RP12345678-2019"}],"crc1610":"AD38","templates":[{"id":80,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123.ABCD.3456.WXYZ"}]}]}';

void main() {
  jsonToBrcode(json);
  // '00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38'
}
```

**Emit**
```dart
import 'package:dartbrcode/dartbrcode.dart';

final brcode = '00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38';

void main() {
  jsonFromBrcode(brcode);
  // '{"payload_version":1,"initiation_method":null,"merchant_account_information":"12345678901234","merchant_information":[{"id":26,"info":[{"id":0,"info":"BR.GOV.BCB.PIX"},{"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}]},{"id":27,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123456789"}]}],"merchant_category_code":0,"merchant_name":"NOME DO RECEBEDOR","merchant_city":"BRASILIA","postal_code":"70074900","currency":"986","amount":123.45,"country_code":"BR","field_template":[{"reference_label":"RP12345678-2019"}],"crc1610":"AD38","templates":[{"id":80,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123.ABCD.3456.WXYZ"}]}]}'
}
```

**Other available functions**:
- `crc16Ccitt`

### Benchmarks

* with `dart_benchmark`
**jsonToBrcode**
```
For 100 runs: peak: 371 us,	bottom: 048 us,	avg: ~083 us
```

**brcodeToJson**
```
For 100 runs: peak: 327 us,	bottom: 069 us,	avg: ~101 us
```

* with `benchmark_harness`
**jsonToBrcode**
```
For 10 runs: 207.51774227018055 us.
```

**brcodeToJson**
```
For 10 runs: 378.68780764861793 us.
```

## Goals
- [x] Parse BR Code String to  `Vec<(usize, Data)>` (more flexible solution);
- [x] Parse BR Code to `BrCode` struct;
- [x] Emit BR Code from `Vec<(usize, Data)>`;
- [x] Emit BR Code from `BrCode` struct;
- [x] CRC16_CCITT
- [x] FFI 
