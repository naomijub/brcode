use crate::parse::Data;

pub(crate) fn emit(code: &[(usize, Data)]) -> String {
    code.iter()
        .map(|e| match &e.1 {
            Data::Single(s) => format!("{:02}{:02}{}", e.0, s.len(), s),
            Data::Vector(v) => {
                let inner = v
                    .iter()
                    .map(|ve| match &ve.1 {
                        Data::Single(vs) => (vs.len(), format!("{:02}{:02}{}", ve.0, vs.len(), vs)),
                        _ => (0, String::new()),
                    })
                    .fold((0usize, String::new()), |acc, e| {
                        (acc.0 + e.0 + 4, acc.1 + &e.1)
                    });
                format!("{:02}{:02}{}", e.0, inner.0, inner.1)
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::emit;
    use crate::from_str;

    #[test]
    fn emits_same_data_as_receives() {
        let code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";
        let parsed_data = from_str(code);
        let emited = emit(&parsed_data);

        assert_eq!(code, emited);
    }

    #[test]
    fn helloworld_in_tag_00() {
        let code = "26062602oi";
        let parsed_data = from_str(code);
        let emited = emit(&parsed_data);

        assert_eq!(code, emited);
    }
}
