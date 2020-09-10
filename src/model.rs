use crate::parse::Data;
#[derive(Debug, PartialEq)]
pub struct BrCode {
    payload_version: u8,
    initiation_methos: Option<u8>,
    // merchant_information: Vec<Info>,
    merchant_category_code: u32,
    // merchant_name: String,
    // merchant_city: String,
    // postal_code: Option<String>,
    // currency: String,
    // amount: Option<f64>,
    // country_code: String,
    // field_template: Vec<Label>,
    // crc1610: String,
    // templates: Option<Vec<(usize, Data)>>
}

pub struct Label {
    reference_label: String,
}

#[derive(Debug, PartialEq)]
pub struct Info {
    id: usize,
    info: String
}

impl From<Vec<(usize, Data)>> for BrCode {
    fn from(code: Vec<(usize, Data)>) -> Self {
        let mut mut_code = code.iter();
        BrCode {
            payload_version: mut_code.find(|e| e.0 == 0usize).unwrap().to_owned().1.to_str().parse().unwrap(),
            initiation_methos: mut_code.find(|e| e.0 == 1usize).map(|(_, code)| (*code).to_str().parse().unwrap()),
            // merchant_information: ,
            merchant_category_code: mut_code.find(|e| e.0 == 0usize).unwrap().to_owned().1.to_str().parse().unwrap(),
            // merchant_name: mut_code.find(|e| e.0 == 59usize).unwrap().1.to_str(),
            // merchant_city: ,
            // postal_code: ,
            // currency: ,
            // amount: ,
            // country_code: ,
            // field_template: ,
            // crc1610: ,
            // templates: ,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elements() {
        assert_eq!(expected(), BrCode::from(code()));
    }

    fn expected() -> BrCode {
        BrCode {
            payload_version: 1,
            initiation_methos: None,
            merchant_category_code: 0000u32,
            // merchant_name: "NOME DO RECEBEDOR".to_string(),
        }
    }
    
    fn code() -> Vec<(usize, Data)> {
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
    
}