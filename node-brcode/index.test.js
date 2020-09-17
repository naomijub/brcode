const brcode = require('./index');

test('Parses and Emit Br Codes', () => {
  expect(brcode.parse(code)).toEqual(json);
  expect(brcode.emit(json)).toEqual(code);
});

test('Crc16 CCITT', () => {
    expect(brcode.crc16Ccitt("biscuit")).toEqual("B8CE");
    expect(brcode.crc16Ccitt(unchecked_code)).toEqual("AD38");
  });

const code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38";
const unchecked_code = "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304";

const json = {
    "payload_version":1,
    "initiation_method":null,
    "merchant_account_information": "12345678901234",
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