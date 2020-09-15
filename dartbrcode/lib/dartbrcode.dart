import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

typedef NativeRustJsonToBrcode = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);
typedef NativeJsonToBrcode = ffi.Pointer<Utf8> Function();
final os = Platform.isMacOS ? "libbrcode.dylib" : Platform.isIOS? "libbrcode.a" : "libbrcode.so";
final ffi.DynamicLibrary dl =
      ffi.DynamicLibrary.open(os);

String jsonToBrcode(String json) {
  final json_to_brcode =
      dl.lookupFunction<NativeRustJsonToBrcode, NativeRustJsonToBrcode>(
          "json_to_brcode");
  final Pointer<Utf8> utf8_brcode = Utf8.toUtf8(json).cast();
  final utf8_from_json = json_to_brcode(utf8_brcode);

  return Utf8.fromUtf8(utf8_from_json).toString();
}

String jsonFromBrcode(String json) {
  final json_from_brcode =
      dl.lookupFunction<NativeRustJsonToBrcode, NativeRustJsonToBrcode>(
          "json_from_brcode");
  final Pointer<Utf8> utf8_brcode = Utf8.toUtf8(json).cast();
  final utf8_to_json = json_from_brcode(utf8_brcode);

  return Utf8.fromUtf8(utf8_to_json).toString();
}