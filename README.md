# BR Code

A crate to parse and emit [PIX BR Code](https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf).

## Usage

```toml
[dependencies]
brcode = "1.0.0"
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
        initiation_methos: None,
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
        initiation_methos: None,
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

## FFI

### Clojure FFI 
[DOCS](https://github.com/naomijub/brcode/blob/master/clj-brcode/README.md)

**BR Code as Edn** call function FFI `edn_from_brcode` or use clojar `[clj-brcode "1.0.0-SNAPSHOT"]`. Example:
```clojure
(ns example.core
  (:require [clj-brcode.core :refer :all]))

(def code "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38")

(brcode->edn code)

; {:payload-version 1, :initiation-methos nil, :merchant-information [{:id 26, :info [{:id 0, :info "BR.GOV.BCB.PIX"}, {:id 1, :info "123e4567-e12b-12d1-a456-426655440000"}]}, {:id 27, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123456789"}]}], :merchant-category-code 0, :merchant-name "NOME DO RECEBEDOR", :merchant-city "BRASILIA", :postal-code "70074900", :currency "986", :amount 123.45, :country-code "BR", :field-template [{:reference-label "RP12345678-2019"}], :crc1610 "AD38", :templates [{:id 80, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123.ABCD.3456.WXYZ"}]}]}
```

Input:
```rust
"00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```

Expected Edn:
```clojure
{ :payload-version 1, 
  :initiation-methos nil, 
  :merchant-information 
    [{ :id 26, :info 
        [{ :id 0, :info "BR.GOV.BCB.PIX", }, 
         { :id 1, :info "123e4567-e12b-12d1-a456-426655440000", }
        ] },
     { :id 27, :info 
        [{ :id 0, :info "BR.COM.OUTRO", }, 
         { :id 1, :info "0123456789", }
        ]}
    ], 
  :merchant-category-code 0, 
  :merchant-name "NOME DO RECEBEDOR", 
  :merchant-city "BRASILIA", 
  :postal-code "70074900", 
  :currency "986", 
  :amount 123.45, 
  :country-code "BR", 
  :field-template [{ :reference-label "RP12345678-2019", }], 
  :crc1610 "AD38", 
  :templates 
    [{ :id 80, :info 
        [{ :id 0, :info "BR.COM.OUTRO", },
         { :id 1, :info "0123.ABCD.3456.WXYZ", }], }], }
```


**Edn as BR Code** call function FFI `edn_to_brcode` or use clojar `[clj-brcode "1.0.0-SNAPSHOT"]`. Example:
```clojure
(ns example.core
  (:require [clj-brcode.core :refer :all]))

(def edn {:payload-version 1, :initiation-methos nil, :merchant-information [{:id 26, :info [{:id 0, :info "BR.GOV.BCB.PIX"}, {:id 1, :info "123e4567-e12b-12d1-a456-426655440000"}]}, {:id 27, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123456789"}]}], :merchant-category-code 0, :merchant-name "NOME DO RECEBEDOR", :merchant-city "BRASILIA", :postal-code "70074900", :currency "986", :amount 123.45, :country-code "BR", :field-template [{:reference-label "RP12345678-2019"}], :crc1610 "AD38", :templates [{:id 80, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123.ABCD.3456.WXYZ"}]}]})

(brcode->edn edn)

; "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```


### Node FFI 
[DOCS](https://github.com/naomijub/brcode/blob/master/node-brcode/README.md)

**BR Code as Json** call function `parse`. Example:
```js
const brcode = require('node-brecode');

const code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";

console.log(brcode.parse(code));
// {
//     "payload_version":1,
//     "initiation_methos":null,
//     "merchant_information":[
//         {"id":26,"info":[
//             {"id":0,"info":"BR.GOV.BCB.PIX"},
//             {"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}
//         ]},
//         {"id":27,"info":[
//             {"id":0,"info":"BR.COM.OUTRO"},
//             {"id":1,"info":"0123456789"}
//         ]}
//     ],
//     "merchant_category_code":0,
//     "merchant_name":"NOME DO RECEBEDOR",
//     "merchant_city":"BRASILIA",
//     "postal_code":"70074900",
//     "currency":"986",
//     "amount":123.45,
//     "country_code":"BR",
//     "field_template":[{"reference_label":"RP12345678-2019"}],
//     "crc1610":"AD38",
//     "templates":[
//         {"id":80,"info":[
//             {"id":0,"info":"BR.COM.OUTRO"},
//             {"id":1,"info":"0123.ABCD.3456.WXYZ"}
//         ]}
//     ]
// }
```

Input:
```rust
"00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```

Expected Json:
```json
{
    "payload_version":1,
    "initiation_methos":null,
    "merchant_information":[
        {"id":26,"info":[
            {"id":0,"info":"BR.GOV.BCB.PIX"},
            {"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}
        ]},
        {"id":27,"info":[
            {"id":0,"info":"BR.COM.OUTRO"},
            {"id":1,"info":"0123456789"}
        ]}
    ],
    "merchant_category_code":0,
    "merchant_name":"NOME DO RECEBEDOR",
    "merchant_city":"BRASILIA",
    "postal_code":"70074900",
    "currency":"986",
    "amount":123.45,
    "country_code":"BR",
    "field_template":[{"reference_label":"RP12345678-2019"}],
    "crc1610":"AD38",
    "templates":[
        {"id":80,"info":[
            {"id":0,"info":"BR.COM.OUTRO"},
            {"id":1,"info":"0123.ABCD.3456.WXYZ"}
        ]}
    ]
}
```

**Json as BR Code** call function `emit`. Example:
```js
const brcode = require('node-brcode');
const json = {
    "payload_version":1,
    "initiation_methos":null,
    "merchant_information":[
        {"id":26,"info":[
            {"id":0,"info":"BR.GOV.BCB.PIX"},
            {"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}
        ]},
        {"id":27,"info":[
            {"id":0,"info":"BR.COM.OUTRO"},
            {"id":1,"info":"0123456789"}
        ]}
    ],
    "merchant_category_code":0,
    "merchant_name":"NOME DO RECEBEDOR",
    "merchant_city":"BRASILIA",
    "postal_code":"70074900",
    "currency":"986",
    "amount":123.45,
    "country_code":"BR",
    "field_template":[{"reference_label":"RP12345678-2019"}],
    "crc1610":"AD38",
    "templates":[
        {"id":80,"info":[
            {"id":0,"info":"BR.COM.OUTRO"},
            {"id":1,"info":"0123.ABCD.3456.WXYZ"}
        ]}
    ]
};

console.log(brcode.emit(json))
// "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
```

## Benchmark

**from_str** in `benches/parse.rs`
```
time:   [16.200 us 16.251 us 16.319 us] 
```

**str_to_brcode** in `benches/to_brcode`
```
time:   [25.424 us 25.570 us 25.710 us]
```

**edn_from_brcode** in `benches/to_brcode`
```
time:   [55.027 us 55.202 us 55.382 us]
```

**json_from_brcode** in `benches/to_brcode`
```
time:   [30.717 us 30.981 us 31.303 us]
```

**both-ways using `BrCode`**
```
time:   [33.238 us 33.555 us 33.924 us]                          
```

**both-ways using `Vec<(usize, Data)>`**
```
time:   [24.537 us 25.391 us 26.260 us] 
```

## Goals
- [x] Parse BR Code String;
- [x] Parse BR Code to `BrCode` struct;
- [x] Emit BR Code from `Vec<(usize, Data)>`;
- [x] Emit BR Code from `BrCode` struct;
- [x] FFI 
    - [x] JS/Node
    - [x] Json
    - [x] Edn
    - [x] Clojure
