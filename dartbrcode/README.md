# dartbrcode

Dart wrapper of `brcode` to parse and emit [PIX BR Code](https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf).

## Usage

1. Include `dartbrcode` in `pubspec.yaml`
```yaml
dependencies:
  dartbrcode: ^0.1.0
```
2. Copy `libbrcode.*` from [brcode](https://github.com/naomijub/brcode) to your Dart/Flutter project root:
  - for Linux/Android copy `libbrcode.so`.
  - for macOS/iOS copy `libbrcode.dylib`.
  - `cargo build --release` project from [git](https://github.com/naomijub/brcode) and copy the `libbrcode.*` from `target/release/libbrcode.*` to your Dart project root. Mobile in the section `Building for mobile`.
  - Shellscript to get files from release:
  **So**
  ```sh
  curl -s https://api.github.com/repos/naomijub/brcode/releases/latest \
  | grep "browser_download_url.*so" \
  | cut -d : -f 2,3 \
  | tr -d \" \
  | wget -qi -
  ```

  **dylib**
  ```sh
  curl -s https://api.github.com/repos/naomijub/brcode/releases/latest \
  | grep "browser_download_url.*dylib" \
  | cut -d : -f 2,3 \
  | tr -d \" \
  | wget -qi -
  ```

3. Use it!

**Parse**
```dart
import 'package:dartbrcode/dartbrcode.dart';

final json = '{"payload_version":1,"initiation_methos":null,"merchant_account_information":"12345678901234","merchant_information":[{"id":26,"info":[{"id":0,"info":"BR.GOV.BCB.PIX"},{"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}]},{"id":27,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123456789"}]}],"merchant_category_code":0,"merchant_name":"NOME DO RECEBEDOR","merchant_city":"BRASILIA","postal_code":"70074900","currency":"986","amount":123.45,"country_code":"BR","field_template":[{"reference_label":"RP12345678-2019"}],"crc1610":"AD38","templates":[{"id":80,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123.ABCD.3456.WXYZ"}]}]}';

void main() {
  jsonToBrcode(json);
  // '00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38'
}
```

**Emit**
```dart
import 'package:dartbrcode/dartbrcode.dart';

final brcode = '00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38';

void main() {
  jsonFromBrcode(brcode);
  // '{"payload_version":1,"initiation_methos":null,"merchant_account_information":"12345678901234","merchant_information":[{"id":26,"info":[{"id":0,"info":"BR.GOV.BCB.PIX"},{"id":1,"info":"123e4567-e12b-12d1-a456-426655440000"}]},{"id":27,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123456789"}]}],"merchant_category_code":0,"merchant_name":"NOME DO RECEBEDOR","merchant_city":"BRASILIA","postal_code":"70074900","currency":"986","amount":123.45,"country_code":"BR","field_template":[{"reference_label":"RP12345678-2019"}],"crc1610":"AD38","templates":[{"id":80,"info":[{"id":0,"info":"BR.COM.OUTRO"},{"id":1,"info":"0123.ABCD.3456.WXYZ"}]}]}'
}
```

## Building for mobile
[Building FFI with Flutter](https://medium.com/flutter-community/using-ffi-on-flutter-plugins-to-run-native-rust-code-d64c0f14f9c2)

### Android
1. Install Android NDK
2. Add Rust target for android `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android`
3. Build cargo for every target
```sh
# $ANDROID_NDK_HOME is already set and pointing to the Android NDK folder

# ENV
AARCH64_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang
ARMV7_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang
I686_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang

# Build
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$AARCH64_LINKER cargo build - target aarch64-linux-android - release
CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$ARMV7_LINKER cargo build - target armv7-linux-androideabi - release
CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$I686_LINKER cargo build - target i686-linux-android - release
```
4. Files will be found at:
```
target/aarch64-linux-android/release/libbrcode.so
target/armv7-linux-androideabi/release/libbrcode.so
target/i686-linux-android/release/libbrcode.so
```

### iOS
1. Instal xcode
2. Add Rust targets for iOS `rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios`
3. Install `cargo-lipo` `cargo install cargo-lipo`.
4. Install `cbindgen` `cargo install cbindgen`.
5. Build targets for iOS  with `cargo lipo --release`.
6. Create a `cbindgen.toml`:
```toml
language = "C"
autogen_warning = "// NOTE: Append the lines below to ios/Classes/Brcode.h"
#namespace = "ffi"
#include_guard = "CBINDGEN_BINDINGS_H"

[defines]
"target_os = ios" = "TARGET_OS_IOS"
"target_os = macos" = "TARGET_OS_MACOS"
```
7. Create C bindigns via:  `cbindgen ./src/lib.rs -c cbindgen.toml | grep -v \#include | uniq`

## Milestones
- [x] parse, `jsonFromBrcode`
- [x] emit, `jsonToBrcode`
- [ ] parse returning `Map<String, dynamic>`
- [ ] emit receiving `Map<String, dynamic>` as args