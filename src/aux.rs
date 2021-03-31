use crate::parse;
use std::{collections::HashMap, iter::FromIterator};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct HashBrCode(pub HashMap<usize, Data>);

impl HashBrCode {
    pub fn new(code: Vec<(usize, parse::Data)>) -> Self {
        code.into_iter().collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Data {
    Str(String),
    Hash(HashBrCode),
}

impl Data {
    pub fn to_str(&self) -> String {
        match self {
            Data::Str(s) => String::from(s),
            _ => String::new(),
        }
    }

    pub fn to_hash(&self) -> HashMap<usize, Data> {
        match self {
            Data::Hash(map) => (map.0).to_owned(),
            _ => HashMap::new(),
        }
    }
}

impl FromIterator<(usize, parse::Data)> for HashBrCode {
    fn from_iter<I>(tuples: I) -> Self
    where
        I: IntoIterator<Item = (usize, parse::Data)>,
    {
        let mut m = HashMap::new();
        for (k, v) in tuples {
            m.entry(k).or_insert(match v {
                parse::Data::Single(s) => Data::Str(s),
                parse::Data::Vector(v) => {
                    let map = v.into_iter().collect();
                    Data::Hash(map)
                }
            });
        }
        Self(m)
    }
}

pub fn crc16_ccitt(message: &str) -> String {
    let mut crc: u16 = 0xFFFF; // initial value
    let polynomial: u16 = 0x1021; // 0001 0000 0010 0001  (0, 5, 12)
    let bytes = message.as_bytes();

    for b in bytes {
        for i in 0u16..8u16 {
            let bit = (b >> (7 - i) & 1) == 1;
            let c15 = (crc >> 15 & 1) == 1;
            crc <<= 1;
            if c15 ^ bit {
                crc ^= polynomial
            };
        }
    }

    crc &= 0xffff;

    format!("{:X}", crc).prepend_remaining_length(4, '0')
}

trait Field {
    fn prepend_remaining_length(&self, length: usize, character: char) -> String;
}

impl Field for String {
    fn prepend_remaining_length(&self, length: usize, character: char) -> String {
        let mut string = self.to_owned();
        let limit = length - string.len();

        for _i in 0..limit {
            string.insert(0, character);
        }

        string
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse as p;

    #[test]
    fn creates_simple_map() {
        let vec = vec![
            (0usize, p::Data::Single("01".to_string())),
            (4usize, p::Data::Single("12345678901234".to_string())),
        ];
        let mut expected = HashMap::new();
        expected.insert(0usize, Data::Str("01".to_string()));
        expected.insert(4usize, Data::Str("12345678901234".to_string()));

        let hash: HashBrCode = vec.into_iter().collect();

        assert_eq!(hash.0, expected);
    }

    #[test]
    fn test_crc16_ccitt() {
        let crc16 = crc16_ccitt("123456789");
        let expected = "29B1";

        assert_eq!(crc16, expected);
    }

    #[test]
    fn creates_nested_map() {
        let vec = vec![
            (0usize, p::Data::Single("01".to_string())),
            (4usize, p::Data::Single("12345678901234".to_string())),
            (
                26,
                p::Data::Vector(vec![
                    (0, p::Data::Single("BR.GOV.BCB.PIX".to_string())),
                    (
                        1,
                        p::Data::Single("123e4567-e12b-12d1-a456-426655440000".to_string()),
                    ),
                ]),
            ),
            (
                27,
                p::Data::Vector(vec![
                    (0, p::Data::Single("BR.COM.OUTRO".to_string())),
                    (1, p::Data::Single("0123456789".to_string())),
                ]),
            ),
            (52, p::Data::Single("0000".to_string())),
        ];
        let mut expected = HashMap::new();
        expected.insert(0usize, Data::Str("01".to_string()));
        expected.insert(4usize, Data::Str("12345678901234".to_string()));
        expected.insert(52usize, Data::Str("0000".to_string()));
        expected.insert(
            26usize,
            Data::Hash({
                let mut m_26 = HashMap::new();
                m_26.insert(0usize, Data::Str("BR.GOV.BCB.PIX".to_string()));
                m_26.insert(
                    1usize,
                    Data::Str("123e4567-e12b-12d1-a456-426655440000".to_string()),
                );
                HashBrCode(m_26)
            }),
        );
        expected.insert(
            27usize,
            Data::Hash({
                let mut m_27 = HashMap::new();
                m_27.insert(0usize, Data::Str("BR.COM.OUTRO".to_string()));
                m_27.insert(1usize, Data::Str("0123456789".to_string()));
                HashBrCode(m_27)
            }),
        );

        let hash: HashBrCode = vec.into_iter().collect();

        assert_eq!(hash.0, expected);
    }

    #[test]
    fn prepends_the_remaining_length() {
        let missing_characters = "123".to_string();
        let with_all_characters = "12345".to_string();

        assert_eq!(missing_characters.prepend_remaining_length(5, '0'), "00123");
        assert_eq!(
            with_all_characters.prepend_remaining_length(5, '0'),
            "12345"
        );
    }
}
