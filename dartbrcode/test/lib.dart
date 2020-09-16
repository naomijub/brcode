import 'package:dartbrcode/dartbrcode.dart';
import 'package:test/test.dart';

const json =
    "{\"payload_version\":1,\"initiation_method\":null,\"merchant_account_information\":\"12345678901234\",\"merchant_information\":[{\"id\":26,\"info\":[{\"id\":0,\"info\":\"BR.GOV.BCB.PIX\"},{\"id\":1,\"info\":\"123e4567-e12b-12d1-a456-426655440000\"}]},{\"id\":27,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123456789\"}]}],\"merchant_category_code\":0,\"merchant_name\":\"NOME DO RECEBEDOR\",\"merchant_city\":\"BRASILIA\",\"postal_code\":\"70074900\",\"currency\":\"986\",\"amount\":123.45,\"country_code\":\"BR\",\"field_template\":[{\"reference_label\":\"RP12345678-2019\"}],\"crc1610\":\"AD38\",\"templates\":[{\"id\":80,\"info\":[{\"id\":0,\"info\":\"BR.COM.OUTRO\"},{\"id\":1,\"info\":\"0123.ABCD.3456.WXYZ\"}]}]}";
const brcode =
    '00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38';

void main() {
  test('generated brcode is correct for json', () {
    final actual = jsonToBrcode(json);
    expect(actual, equals(brcode));
  });

  test('generated json is correct for brcode', () {
    final actual = brcodeToJson(brcode);
    expect(actual, equals(json));
  });
}
