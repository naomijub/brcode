import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

typedef NativeRustJsonToBrcode = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);
typedef NativeJsonToBrcode = ffi.Pointer<Utf8> Function();

typedef NativeRustBrcodeToJson = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);
typedef NativeBrcodeToJson = ffi.Pointer<Utf8> Function();

final osSpecificFile = Platform.isMacOS
    ? "libbrcode.dylib"
    : Platform.isIOS ? "libbrcode.a" : "libbrcode.so";
final ffi.DynamicLibrary dl = ffi.DynamicLibrary.open(osSpecificFile);

String jsonToBrcode(String json) {
  final json_to_brcode =
      dl.lookupFunction<NativeRustJsonToBrcode, NativeRustJsonToBrcode>(
          "json_to_brcode");
  final Pointer<Utf8> utf8_brcode = Utf8.toUtf8(json).cast();
  final utf8_from_json = json_to_brcode(utf8_brcode);

  return Utf8.fromUtf8(utf8_from_json).toString();
}

String brcodeToJson(String code) {
  final json_from_brcode =
      dl.lookupFunction<NativeRustBrcodeToJson, NativeRustBrcodeToJson>(
          "json_from_brcode");
  final Pointer<Utf8> utf8_brcode = Utf8.toUtf8(code).cast();
  final utf8_to_json = json_from_brcode(utf8_brcode);

  return Utf8.fromUtf8(utf8_to_json).toString();
}

String crc16Ccitt(String code) {
  final crc16_ccitt_from_message =
      dl.lookupFunction<NativeRustJsonToBrcode, NativeRustJsonToBrcode>(
          "crc16_ccitt_from_message");
  final Pointer<Utf8> utf8_message = Utf8.toUtf8(code).cast();
  final checksum = crc16_ccitt_from_message(utf8_message);

  return Utf8.fromUtf8(checksum).toString();
}
