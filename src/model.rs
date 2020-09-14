use crate::aux::HashBrCode;
use crate::parse::Data;
use edn_derive::{Deserialize, Serialize};
use serde_derive::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SerdeDeserialize, SerdeSerialize)]
pub struct BrCode {
    pub payload_version: u8,
    pub initiation_methos: Option<u8>,
    pub merchant_account_information: Option<String>,
    pub merchant_information: Vec<MerchantInfo>,
    pub merchant_category_code: u32,
    pub merchant_name: String,
    pub merchant_city: String,
    pub postal_code: Option<String>,
    pub currency: String,
    pub amount: Option<f64>,
    pub country_code: String,
    pub field_template: Vec<Label>,
    pub crc1610: String,
    pub templates: Option<Vec<Template>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SerdeDeserialize, SerdeSerialize)]
pub struct Label {
    pub reference_label: String,
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
            .map(|i| (i, hash.get(&i).clone()))
            .filter_map(|e| {
                e.1.map(|d| {
                    Some(MerchantInfo {
                        id: e.0,
                        info: {
                            let hm = d.to_hash();
                            let mut keys = hm.keys().map(|k| *k).collect::<Vec<usize>>();
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
            .map(|i| (i, hash.get(&i).clone()))
            .filter_map(|e| {
                e.1.map(|d| {
                    Some(Template {
                        id: e.0,
                        info: {
                            let hm = d.to_hash();
                            let mut keys = hm.keys().map(|k| *k).collect::<Vec<usize>>();
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
            initiation_methos: hash.get(&1usize).map(|e| e.to_str().parse().unwrap()),
            merchant_account_information: hash.get(&4usize).map(|e| e.to_str()),
            merchant_information: merchant_information,
            merchant_category_code: hash[&52usize].to_str().parse().unwrap(),
            merchant_name: hash[&59usize].to_str(),
            merchant_city: hash[&60usize].to_str(),
            postal_code: hash.get(&61usize).map(|e| e.to_str()),
            currency: hash[&53usize].to_str(),
            amount: hash.get(&54usize).map(|e| e.to_str().parse().unwrap()),
            country_code: hash[&58usize].to_str(),
            field_template: vec![Label {
                reference_label: hash[&62usize].to_hash()[&5usize].to_str(),
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
    pub fn encode(self) -> String {
        let mut encode = String::new();
        encode.push_str(&format!("0002{:02}", self.payload_version));
        match self.initiation_methos {
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
        encode.push_str(&format!("5303{:02}", self.currency));
        match self.amount {
            None => (),
            Some(a) => encode.push_str(&format!("54{:02}{}", a.to_string().len(), a)),
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
        encode.push_str(&format!(
            "62{:02}{}",
            field_template.len() + 4,
            format!("05{:02}{}", field_template.len(), field_template)
        ));
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

    fn expected() -> BrCode {
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

    fn encoded() -> String {
        "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
        .to_string()
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
