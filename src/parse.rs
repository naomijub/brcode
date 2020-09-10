use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Data {
    Single(String),
    Vector(Vec<(usize, Data)>),
}

pub fn parse(code: &str) -> Vec<(usize, Data)> {
    let mut chars = code.chars();
    (0usize..=99usize)
        .filter_map(|_| parse_code(&mut chars))
        .map(|code| match code.0 {
            26..=51 => (code.0, Data::Vector(parse(&code.1))),
            80..=99 => (code.0, Data::Vector(parse(&code.1))),
            62 => (code.0, Data::Vector(parse(&code.1))),
            _ => (code.0, Data::Single(code.1)),
        })
        .collect()
}

fn parse_code(chars: &mut Chars) -> Option<(usize, String)> {
    match (
        chars.take(2).collect::<String>().parse(),
        chars.take(2).collect::<String>().parse(),
    ) {
        (Ok(id), Ok(len)) => {
            let value: String = chars.take(len).collect();
            Some((id, value))
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::{parse, Data};

    #[test]
    fn helloworld_in_tag_00() {
        let code = "0011hello-world";
        let expected = vec![(0usize, Data::Single(String::from("hello-world")))];

        assert_eq!(parse(code), expected);
    }

    #[test]
    fn code_with_inner_values() {
        let code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-426655440000";
        let expected = vec![
            (0usize, Data::Single("01".to_string())),
            (4usize, Data::Single("12345678901234".to_string())),
            (
                26,
                Data::Vector(vec![
                    (0usize, Data::Single("BR.GOV.BCB.PIX".to_string())),
                    (
                        1usize,
                        Data::Single("123e4567-e12b-12d1-a456-426655440000".to_string()),
                    ),
                ]),
            ),
        ];

        assert_eq!(parse(code), expected);
    }
}
