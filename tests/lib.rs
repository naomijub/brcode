use brcode::{
    self, crc16_ccitt_from_message, edn_from_brcode, edn_to_brcode, from_str, json_from_brcode,
    json_to_brcode, str_to_brcode, BrCode, Data, Info, Label, MerchantInfo, Template,
};

#[test]
fn test_from_str() {
    assert_eq!(from_str(&code(None, None)), data_expected());
}

#[test]
fn test_to_string() {
    let actual = brcode::to_string(&data_expected());
    let expected = code(None, None);

    assert_eq!(actual, expected);
}

#[test]
fn assert_both_ways() {
    let from = brcode::from_str(&dynamic_code());
    let to = brcode::to_string(&from);

    assert_eq!(to, dynamic_code())
}

#[test]
fn test_brcode_to_string() {
    let actual = brcode::brcode_to_string(brcode_expected(None));
    let expected = code(None, None);

    assert_eq!(actual, expected);
}

#[test]
fn test_brcode_to_string_cents_edge_case() {
    let actual = brcode::brcode_to_string(brcode_expected(Some(1.1)));
    let expected = code(Some(1.1), Some("A6C1"));

    assert_eq!(actual, expected);
}

#[test]
fn test_str_to_brcode() {
    assert_eq!(str_to_brcode(&code(None, None)), brcode_expected(None));
}

#[test]
fn minimum_breaking_code() {
    let code = "26062602oi";
    let expected = vec![(
        26usize,
        Data::Vector(vec![(26usize, Data::Single("oi".to_string()))]),
    )];
    assert_eq!(from_str(code), expected);
}

#[test]
fn brcode_is_pix() {
    let from = str_to_brcode(&code(None, None));

    assert!(from.is_pix())
}

#[test]
fn brcode_label() {
    let from = str_to_brcode(&code(None, None));

    assert_eq!(
        from.get_transaction_id(),
        Some("RP12345678-2019".to_string())
    )
}

#[test]
fn brcode_get_alias() {
    let from = str_to_brcode(&brcode_with_alias_message());

    assert_eq!(from.get_alias(), Some(vec!["11999887766".to_string()]))
}

#[test]
fn brcode_get_message() {
    let from = str_to_brcode(&brcode_with_alias_message());

    assert_eq!(from.get_message(), Some(vec!["Hello message".to_string()]))
}

#[test]
fn json_ffi() {
    let code = code(None, None);
    let result = json_from_brcode(to_c_char(code));
    let actual = to_string(result);

    assert_eq!(actual, json());
}

#[test]
fn edn_ffi() {
    let code = code(None, None);
    let result = edn_from_brcode(to_c_char(code));
    let actual = to_string(result);

    assert_eq!(actual, edn());
}

#[test]
fn edn_to_brcode_ffi() {
    let edn = edn();
    let code = code(None, None);
    let result = edn_to_brcode(to_c_char(edn));
    let actual = to_string(result);

    assert_eq!(actual, code);
}

#[test]
fn json_to_brcode_ffi() {
    let json = json();
    let code = code(None, None);
    let result = json_to_brcode(to_c_char(json));
    let actual = to_string(result);

    assert_eq!(actual, code);
}

#[test]
fn crc16_test() {
    let message = "00020101021226740014br.gov.bcb.spi210812345678220412342308123456782420001122334455667788995204000053039865406123.455802BR5913FULANO DE TAL6008BRASILIA62190515RP12345678-201980720014br.gov.bcb.spi2550bx.com.br/spi/U0VHUkVET1RPVEFMTUVOVEVBTEVBVE9SSU8=6304";
    let expected = "34D1";

    let result = crc16_ccitt_from_message(to_c_char(message.to_string()));
    let actual = to_string(result);

    assert_eq!(actual, expected);
}

fn dynamic_code() -> String {
    "00020101021226740014br.gov.bcb.spi210812345678220412342308123456782420001122334455667788995204000053039865406123.455802BR5913FULANO DE TAL6008BRASILIA62190515RP12345678-201980720014br.gov.bcb.spi2550bx.com.br/spi/U0VHUkVET1RPVEFMTUVOVEVBTEVBVE9SSU8=630434D1"
    .to_string()
}

fn code(amount: Option<f64>, crc16: Option<&str>) -> String {
    let amount = amount.unwrap_or(123.45);
    let amount_size = amount.to_string().len();
    format!("00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO0110012345678952040000530398654{:02}{:.2}5802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304{}", amount_size, amount, crc16.unwrap_or("AD38"))
}

fn json() -> String {
    "{\"payload_version\":1,\"merchant_account_information\":\"12345678901234\",\"merchant_information\":[{\"id\":26,\"info\":[{\"id\":0,\"info\":\"BR.GOV.BCB.PIX\"},{\"id\":1,\"info\":\"123e4567-e12b-12d1-a456-426655440000\"}]},{\"id\":27,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123456789\"}]}],\"merchant_category_code\":0,\"merchant_name\":\"NOME DO RECEBEDOR\",\"merchant_city\":\"BRASILIA\",\"postal_code\":\"70074900\",\"currency\":\"986\",\"amount\":123.45,\"country_code\":\"BR\",\"field_template\":[{\"reference_label\":\"RP12345678-2019\"}],\"crc1610\":\"AD38\",\"templates\":[{\"id\":80,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123.ABCD.3456.WXYZ\"}]}]}"
    .to_string()
}

fn edn() -> String {
    "{ :payload-version 1, :initiation-method nil, :merchant-account-information \"12345678901234\", :merchant-information [{ :id 26, :info [{ :id 0, :info \"BR.GOV.BCB.PIX\", }, { :id 1, :info \"123e4567-e12b-12d1-a456-426655440000\", }], }, { :id 27, :info [{ :id 0, :info \"BR.COM.OUTRO\", }, { :id 1, :info \"0123456789\", }], }], :merchant-category-code 0, :merchant-name \"NOME DO RECEBEDOR\", :merchant-city \"BRASILIA\", :convenience nil, :convenience-fee-fixed nil, :convenience-fee-percentage nil, :postal-code \"70074900\", :currency \"986\", :amount 123.45, :country-code \"BR\", :field-template [{ :reference-label \"RP12345678-2019\", }], :crc1610 \"AD38\", :templates [{ :id 80, :info [{ :id 0, :info \"BR.COM.OUTRO\", }, { :id 1, :info \"0123.ABCD.3456.WXYZ\", }], }], }"
    .to_string()
}

fn brcode_expected(amount: Option<f64>) -> BrCode {
    BrCode {
        payload_version: 1,
        initiation_method: None,
        merchant_account_information: Some(String::from("12345678901234")),
        merchant_category_code: 0000u32,
        merchant_name: "NOME DO RECEBEDOR".to_string(),
        merchant_city: "BRASILIA".to_string(),
        convenience: None,
        convenience_fee_fixed: None,
        convenience_fee_percentage: None,
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
        amount: amount.or(Some(123.45)),
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

fn data_expected() -> Vec<(usize, Data)> {
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

fn brcode_with_alias_message() -> String {
    "00020104141234567890123426500014BR.GOV.BCB.PIX0111119998877660213Hello message27300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ63049059"
    .to_string()
}

// FFI Tests
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::str;

fn to_string(pointer: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    str::from_utf8(slice).unwrap().to_string()
}

fn to_c_char(s: String) -> *const c_char {
    let cs = CString::new(s.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    mem::forget(cs);
    ptr
}
