use crate::aux::HashBrCode;
use crate::parse::Data;
use edn_derive::{Deserialize, Serialize};
use qrcode_generator::QrCodeEcc;
use serde_derive::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SerdeDeserialize, SerdeSerialize)]
pub struct BrCode {
    pub payload_version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_method: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_account_information: Option<String>,
    pub merchant_information: Vec<MerchantInfo>,
    pub merchant_category_code: u32,
    pub merchant_name: String,
    pub merchant_city: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convenience: Option<String>, //{pub type 55 pub kind pub scalar}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convenience_fee_fixed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] // {pub type 56 pub kind pub scalar}
    pub convenience_fee_percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] // {pub type 57 pub kind pub scalar}
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    pub country_code: String,
    pub field_template: Vec<Label>,
    pub crc1610: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<Template>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SerdeDeserialize, SerdeSerialize)]
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_label: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SerdeDeserialize, SerdeSerialize)]
pub struct MerchantInfo {
    pub id: usize,
    pub info: Vec<Info>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SerdeDeserialize, SerdeSerialize)]
pub struct Template {
    pub id: usize,
    pub info: Vec<Info>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SerdeDeserialize, SerdeSerialize)]
pub struct Info {
    pub id: usize,
    pub info: String,
}

impl std::fmt::Display for BrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone().encode())
    }
}

impl From<Vec<(usize, Data)>> for BrCode {
    fn from(code: Vec<(usize, Data)>) -> Self {
        let hash = HashBrCode::new(code).0;
        let merchant_information = (26usize..=51usize)
            .map(|i| (i, hash.get(&i)))
            .filter_map(|e| {
                e.1.map(|d| {
                    Some(MerchantInfo {
                        id: e.0,
                        info: {
                            let hm = d.to_hash();
                            let mut keys = hm.keys().copied().collect::<Vec<usize>>();
                            keys.sort();
                            keys.into_iter()
                                .map(|idx| Info {
                                    id: idx,
                                    info: hm[&idx].to_str(),
                                })
                                .collect::<Vec<Info>>()
                        },
                    })
                })?
            })
            .collect::<Vec<MerchantInfo>>();
        let templates = (80usize..=99usize)
            .map(|i| (i, hash.get(&i)))
            .filter_map(|e| {
                e.1.map(|d| {
                    Some(Template {
                        id: e.0,
                        info: {
                            let hm = d.to_hash();
                            let mut keys = hm.keys().copied().collect::<Vec<usize>>();
                            keys.sort();
                            keys.into_iter()
                                .map(|idx| Info {
                                    id: idx,
                                    info: hm[&idx].to_str(),
                                })
                                .collect::<Vec<Info>>()
                        },
                    })
                })?
            })
            .collect::<Vec<Template>>();

        BrCode {
            payload_version: hash[&0usize].to_str().parse().unwrap(),
            initiation_method: hash.get(&1usize).map(|e| e.to_str().parse().unwrap()),
            merchant_account_information: hash.get(&4usize).map(crate::aux::Data::to_str),
            merchant_information: merchant_information,
            merchant_category_code: hash[&52usize].to_str().parse().unwrap(),
            merchant_name: hash[&59usize].to_str(),
            merchant_city: hash[&60usize].to_str(),
            postal_code: hash.get(&61usize).map(crate::aux::Data::to_str),
            currency: hash.get(&53usize).map(|e| e.to_str().parse().unwrap()),
            amount: hash.get(&54usize).map(|e| e.to_str().parse().unwrap()),
            convenience: hash.get(&55usize).map(crate::aux::Data::to_str),
            convenience_fee_fixed: hash.get(&56usize).map(crate::aux::Data::to_str),
            convenience_fee_percentage: hash.get(&67usize).map(crate::aux::Data::to_str),
            country_code: hash[&58usize].to_str(),
            field_template: vec![Label {
                reference_label: hash.get(&62usize).map(|e| e.to_hash()[&5usize].to_str()),
            }],
            crc1610: hash[&63usize].to_str(),
            templates: if templates.is_empty() {
                None
            } else {
                Some(templates)
            },
        }
    }
}

impl BrCode {
    pub fn is_pix(&self) -> bool {
        self.merchant_information
            .iter()
            .filter(|e| e.id >= 26 && e.id <= 51)
            .any(|e| {
                e.info
                    .iter()
                    .filter(|i| i.id == 0)
                    .any(|i| i.info.to_uppercase() == "BR.GOV.BCB.PIX")
            })
    }

    pub fn get_transaction_id(&self) -> Option<String> {
        self.field_template.first()?.reference_label.clone()
    }

    pub fn get_alias(&self) -> Option<Vec<String>> {
        if self.is_pix() {
            Some(
                self.merchant_information
                    .iter()
                    .filter(|e| e.id >= 26 && e.id <= 51)
                    .flat_map(|e| {
                        e.info
                            .iter()
                            .filter(|i| {
                                i.id == 1 && e.info.first().unwrap().info == "BR.GOV.BCB.PIX"
                            })
                            .map(|i| i.info.clone())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<String>>(),
            )
        } else {
            None
        }
    }

    pub fn get_message(&self) -> Option<Vec<String>> {
        if self.is_pix() {
            Some(
                self.merchant_information
                    .iter()
                    .filter(|e| e.id >= 26 && e.id <= 51)
                    .flat_map(|e| {
                        e.info
                            .iter()
                            .filter(|i| {
                                i.id == 2 && e.info.first().unwrap().info == "BR.GOV.BCB.PIX"
                            })
                            .map(|i| i.info.clone())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<String>>(),
            )
        } else {
            None
        }
    }

    pub fn encode(self) -> String {
        let mut encode = String::new();
        encode.push_str(&format!("0002{:02}", self.payload_version));
        match self.initiation_method {
            None => (),
            Some(m) => encode.push_str(&format!("0102{:02}", m)),
        }
        match self.merchant_account_information {
            None => (),
            Some(m) => encode.push_str(&format!("0414{:02}", m)),
        }
        //26 -51
        self.merchant_information.iter().for_each(|m| {
            let id = m.id;
            let inner_size: usize = m.info.iter().map(|i| i.info.len() + 4).sum();
            encode.push_str(&format!("{:02}{:02}", id, inner_size));
            m.info.iter().for_each(|i| {
                encode.push_str(&format!("{:02}{:02}{}", i.id, i.info.len(), i.info));
            });
        });
        encode.push_str(&format!("5204{:04}", self.merchant_category_code));
        match self.currency {
            None => (),
            Some(c) => encode.push_str(&format!("5303{:02}", c)),
        }
        match self.amount {
            None => (),
            Some(a) => encode.push_str(&format!("54{:02}{}", a.to_string().len(), a)),
        }
        match self.convenience {
            None => (),
            Some(c) => encode.push_str(&format!("5502{}", c)),
        }
        match self.convenience_fee_fixed {
            None => (),
            Some(c) => encode.push_str(&format!("56{:02}{}", c.to_string().len(), c)),
        }
        match self.convenience_fee_percentage {
            None => (),
            Some(c) => encode.push_str(&format!("57{:02}{}", c.to_string().len(), c)),
        }
        encode.push_str(&format!("5802{}", self.country_code));
        encode.push_str(&format!(
            "59{:02}{}",
            self.merchant_name.len(),
            self.merchant_name
        ));
        encode.push_str(&format!(
            "60{:02}{}",
            self.merchant_city.len(),
            self.merchant_city
        ));
        match self.postal_code {
            None => (),
            Some(p) => encode.push_str(&format!("61{:02}{}", p.to_string().len(), p)),
        }
        let field_template = self.field_template[0].reference_label.clone();
        match field_template {
            None => (),
            Some(f) => encode.push_str(&format!(
                "62{:02}{}",
                f.len() + 4,
                format!("05{:02}{}", f.len(), f)
            )),
        }
        //80-99
        match self.templates {
            None => (),
            Some(template) => template.iter().for_each(|m| {
                let id = m.id;
                let inner_size: usize = m.info.iter().map(|i| i.info.len() + 4).sum();
                encode.push_str(&format!("{:02}{:02}", id, inner_size));
                m.info.iter().for_each(|i| {
                    encode.push_str(&format!("{:02}{:02}{}", i.id, i.info.len(), i.info));
                });
            }),
        }
        encode.push_str("6304");
        let crc16 = crate::aux::crc16_ccitt(&encode);
        encode.push_str(&crc16);
        encode
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or("ERROR".to_string())
    }

    fn pre_encode(&self) -> String {
        let mut encode = String::new();
        encode.push_str(&format!("0002{:02}", self.payload_version));
        match self.initiation_method {
            None => (),
            Some(m) => encode.push_str(&format!("0102{:02}", m)),
        }
        match self.merchant_account_information.clone() {
            None => (),
            Some(m) => encode.push_str(&format!("0414{:02}", m)),
        }
        //26 -51
        self.merchant_information.iter().for_each(|m| {
            let id = m.id;
            let inner_size: usize = m.info.iter().map(|i| i.info.len() + 4).sum();
            encode.push_str(&format!("{:02}{:02}", id, inner_size));
            m.info.iter().for_each(|i| {
                encode.push_str(&format!("{:02}{:02}{}", i.id, i.info.len(), i.info));
            });
        });
        encode.push_str(&format!("5204{:04}", self.merchant_category_code));
        match self.currency.clone() {
            None => (),
            Some(c) => encode.push_str(&format!("5303{:02}", c)),
        }
        match self.amount {
            None => (),
            Some(a) => encode.push_str(&format!("54{:02}{}", a.to_string().len(), a)),
        }
        match self.convenience.clone() {
            None => (),
            Some(c) => encode.push_str(&format!("5502{}", c)),
        }
        match self.convenience_fee_fixed.clone() {
            None => (),
            Some(c) => encode.push_str(&format!("56{:02}{}", c.to_string().len(), c)),
        }
        match self.convenience_fee_percentage.clone() {
            None => (),
            Some(c) => encode.push_str(&format!("57{:02}{}", c.to_string().len(), c)),
        }
        encode.push_str(&format!("5802{}", self.country_code));
        encode.push_str(&format!(
            "59{:02}{}",
            self.merchant_name.len(),
            self.merchant_name
        ));
        encode.push_str(&format!(
            "60{:02}{}",
            self.merchant_city.len(),
            self.merchant_city
        ));
        match self.postal_code.clone() {
            None => (),
            Some(p) => encode.push_str(&format!("61{:02}{}", p.to_string().len(), p)),
        }
        let field_template = self.field_template[0].reference_label.clone();
        match field_template {
            None => (),
            Some(f) => encode.push_str(&format!(
                "62{:02}{}",
                f.len() + 4,
                format!("05{:02}{}", f.len(), f)
            )),
        }
        //80-99
        match self.templates.clone() {
            None => (),
            Some(template) => template.iter().for_each(|m| {
                let id = m.id;
                let inner_size: usize = m.info.iter().map(|i| i.info.len() + 4).sum();
                encode.push_str(&format!("{:02}{:02}", id, inner_size));
                m.info.iter().for_each(|i| {
                    encode.push_str(&format!("{:02}{:02}{}", i.id, i.info.len(), i.info));
                });
            }),
        }
        encode.push_str("6304");
        encode
    }

    pub fn to_crc16_json(mut self) -> String {
        let encode = self.pre_encode();
        let crc16 = crate::aux::crc16_ccitt(&encode);
        self.crc1610 = crc16;
        serde_json::to_string(&self).unwrap_or("ERROR".to_string())
    }

    pub fn to_svg_string(&self, ecc: QrCodeEcc, size: usize) -> String {
        let brcode = self.clone().encode();
        let result: String =
            qrcode_generator::to_svg_to_string(&brcode.clone(), ecc, size, Some(brcode)).unwrap();
        result
    }

    pub fn to_svg_standard_string(&self) -> String {
        let brcode = self.clone().encode();
        let result: String =
            qrcode_generator::to_svg_to_string(&brcode.clone(), QrCodeEcc::Low, 1024, Some(brcode))
                .unwrap();
        result
    }

    pub fn to_vec_u8(&self, ecc: QrCodeEcc, size: usize) -> Vec<u8> {
        let brcode = self.clone().encode();
        let result = qrcode_generator::to_png_to_vec(&brcode, ecc, size).unwrap();
        result
    }

    pub fn to_svg_file(&self, file_path: &str, ecc: QrCodeEcc, size: usize) {
        let brcode = self.clone().encode();
        qrcode_generator::to_svg_to_file(&brcode.clone(), ecc, size, Some(brcode), file_path)
            .unwrap();
    }

    pub fn to_standard_svg_file(&self, file_path: &str) {
        let brcode = self.clone().encode();
        qrcode_generator::to_svg_to_file(
            &brcode.clone(),
            QrCodeEcc::Low,
            1024,
            Some(brcode),
            file_path,
        )
        .unwrap();
    }

    pub fn to_png_file(&self, file_path: &str, ecc: QrCodeEcc, size: usize) {
        let brcode = self.clone().encode();
        qrcode_generator::to_png_to_file(&brcode.clone(), ecc, size, file_path).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elements() {
        assert_eq!(expected(), BrCode::from(code()));
    }

    #[test]
    fn enconde_brcode() {
        let actual = brcode_value().encode();
        let expected = encoded();

        assert_eq!(actual, expected);
    }

    #[test]
    fn brcode_to_svg() {
        let svg = expected().to_svg_standard_string();
        assert_eq!(&svg[38..42], "<svg");
    }

    #[test]
    fn json_pure() {
        let brcode = brcode_value();
        let json = brcode.to_json();
        let expected = "{\"payload_version\":1,\"merchant_account_information\":\"12345678901234\",\"merchant_information\":[{\"id\":26,\"info\":[{\"id\":0,\"info\":\"BR.GOV.BCB.PIX\"},{\"id\":1,\"info\":\"123e4567-e12b-12d1-a456-426655440000\"}]},{\"id\":27,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123456789\"}]}],\"merchant_category_code\":0,\"merchant_name\":\"NOME DO RECEBEDOR\",\"merchant_city\":\"BRASILIA\",\"postal_code\":\"70074900\",\"currency\":\"986\",\"amount\":123.45,\"country_code\":\"BR\",\"field_template\":[{\"reference_label\":\"RP12345678-2019\"}],\"crc1610\":\"AE38\",\"templates\":[{\"id\":80,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123.ABCD.3456.WXYZ\"}]}]}";

        assert_eq!(json, expected);
    }

    #[test]
    fn json_crc16() {
        let brcode = brcode_value();
        let json = brcode.to_crc16_json();
        let expected = "{\"payload_version\":1,\"merchant_account_information\":\"12345678901234\",\"merchant_information\":[{\"id\":26,\"info\":[{\"id\":0,\"info\":\"BR.GOV.BCB.PIX\"},{\"id\":1,\"info\":\"123e4567-e12b-12d1-a456-426655440000\"}]},{\"id\":27,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123456789\"}]}],\"merchant_category_code\":0,\"merchant_name\":\"NOME DO RECEBEDOR\",\"merchant_city\":\"BRASILIA\",\"postal_code\":\"70074900\",\"currency\":\"986\",\"amount\":123.45,\"country_code\":\"BR\",\"field_template\":[{\"reference_label\":\"RP12345678-2019\"}],\"crc1610\":\"AD38\",\"templates\":[{\"id\":80,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123.ABCD.3456.WXYZ\"}]}]}";

        assert_eq!(json, expected);
    }

    fn expected() -> BrCode {
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

    fn encoded() -> String {
        "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
        .to_string()
    }

    fn brcode_value() -> BrCode {
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
            crc1610: "AE38".to_string(),
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
