use brcode::{
    self, crc16_ccitt_from_message, edn_from_brcode, edn_to_brcode, from_str, json_from_brcode,
    json_to_brcode, str_to_brcode, read_qrcode, read_qrcode_as_brcode, BrCode, Data, Info, Label, MerchantInfo, Template,
};

#[test]
fn test_from_str() {
    assert_eq!(from_str(&code()), data_expected());
}

#[test]
fn test_to_string() {
    let actual = brcode::to_string(&data_expected());
    let expected = code();

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
    let actual = brcode::brcode_to_string(brcode_expected());
    let expected = code();

    assert_eq!(actual, expected);
}

#[test]
fn test_str_to_brcode() {
    assert_eq!(str_to_brcode(&code()), brcode_expected());
}

#[test]
fn haha() {
    let c = str_to_brcode("00020101021126440014br.gov.bcb.spi0122fulano2019@example.com5204000053039865802BR5913FULANO DE TAL6008BRASILIA6304DFE3");
    let d = brcode_expected();
    assert_eq!(c, d);
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
    let from = str_to_brcode(&code());

    assert!(from.is_pix())
}

#[test]
fn brcode_label() {
    let from = str_to_brcode(&code());

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
fn read_a_brcode_image_to_str() {
    let brcode_str = read_qrcode("qrcode.png".to_string());

    assert_eq!(
        brcode_str, 
        vec!["00020101021126440014br.gov.bcb.spi0122fulano2019@example.com5204000053039865802BR5913FULANO DE TAL6008BRASILIA6304DFE3"]);
}

#[test]
fn read_a_brcode_image_to_brcode() {
    let brcode_str = read_qrcode_as_brcode("qrcode.png".to_string());

    assert_eq!(
        brcode_str, 
        vec![brcode_expected()]);
}

#[test]
fn brcode_get_message() {
    let from = str_to_brcode(&brcode_with_alias_message());

    assert_eq!(from.get_message(), Some(vec!["Hello message".to_string()]))
}

#[test]
fn json_ffi() {
    let code = code();
    let result = json_from_brcode(to_c_char(code));
    let actual = to_string(result);

    assert_eq!(actual, json());
}

#[test]
fn edn_ffi() {
    let code = code();
    let result = edn_from_brcode(to_c_char(code));
    let actual = to_string(result);

    assert_eq!(actual, edn());
}

#[test]
fn edn_to_brcode_ffi() {
    let edn = edn();
    let code = code();
    let result = edn_to_brcode(to_c_char(edn));
    let actual = to_string(result);

    assert_eq!(actual, code);
}

#[test]
fn json_to_brcode_ffi() {
    let json = json();
    let code = code();
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

fn code() -> String {
    "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
    .to_string()
}

fn json() -> String {
    "{\"payload_version\":1,\"merchant_account_information\":\"12345678901234\",\"merchant_information\":[{\"id\":26,\"info\":[{\"id\":0,\"info\":\"BR.GOV.BCB.PIX\"},{\"id\":1,\"info\":\"123e4567-e12b-12d1-a456-426655440000\"}]},{\"id\":27,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123456789\"}]}],\"merchant_category_code\":0,\"merchant_name\":\"NOME DO RECEBEDOR\",\"merchant_city\":\"BRASILIA\",\"postal_code\":\"70074900\",\"currency\":\"986\",\"amount\":123.45,\"country_code\":\"BR\",\"field_template\":[{\"reference_label\":\"RP12345678-2019\"}],\"crc1610\":\"AD38\",\"templates\":[{\"id\":80,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123.ABCD.3456.WXYZ\"}]}]}"
    .to_string()
}

fn edn() -> String {
    "{ :payload-version 1, :initiation-method nil, :merchant-account-information \"12345678901234\", :merchant-information [{ :id 26, :info [{ :id 0, :info \"BR.GOV.BCB.PIX\", }, { :id 1, :info \"123e4567-e12b-12d1-a456-426655440000\", }], }, { :id 27, :info [{ :id 0, :info \"BR.COM.OUTRO\", }, { :id 1, :info \"0123456789\", }], }], :merchant-category-code 0, :merchant-name \"NOME DO RECEBEDOR\", :merchant-city \"BRASILIA\", :convenience nil, :convenience-fee-fixed nil, :convenience-fee-percentage nil, :postal-code \"70074900\", :currency \"986\", :amount 123.45, :country-code \"BR\", :field-template [{ :reference-label \"RP12345678-2019\", }], :crc1610 \"AD38\", :templates [{ :id 80, :info [{ :id 0, :info \"BR.COM.OUTRO\", }, { :id 1, :info \"0123.ABCD.3456.WXYZ\", }], }], }"
    .to_string()
}

fn brcode_expected() -> BrCode {
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
        currency: Some("986".to_string()),
        postal_code: Some("70074900".to_string()),
        amount: Some(123.45),
        country_code: "BR".to_string(),
        field_template: vec![Label {
            reference_label: Some("RP12345678-2019".to_string()),
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
