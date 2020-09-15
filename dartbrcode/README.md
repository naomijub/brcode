# node-brcode

Dart wrapper of `brcode` to parse and emit [PIX BR Code](https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf).

## Usage

1. Include `dartbrcode` in `pub.yaml`
```yaml
dependencies:
  dartbrcode: ^1.0.0
```
2. Copy `libbrcode.*` from [brcode](https://github.com/naomijub/brcode/tree/master/clj-brcode) to your Node project root:
  - for linux copy `libbrcode.so`.
  - for macos copy `libbrcode.dylib`.
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
4. Arquivos estarão localizado em:
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

## License
                   GNU LESSER GENERAL PUBLIC LICENSE
                       Version 3, 29 June 2007

 Copyright (C) 2007 Free Software Foundation, Inc. [http://fsf.org/]
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.


  This version of the GNU Lesser General Public License incorporates
the terms and conditions of version 3 of the GNU General Public
License, supplemented by the additional permissions listed below.

  0. Additional Definitions.

  As used herein, "this License" refers to version 3 of the GNU Lesser
General Public License, and the "GNU GPL" refers to version 3 of the GNU
General Public License.

  "The Library" refers to a covered work governed by this License,
other than an Application or a Combined Work as defined below.

  An "Application" is any work that makes use of an interface provided
by the Library, but which is not otherwise based on the Library.
Defining a subclass of a class defined by the Library is deemed a mode
of using an interface provided by the Library.

  A "Combined Work" is a work produced by combining or linking an
Application with the Library.  The particular version of the Library
with which the Combined Work was made is also called the "Linked
Version".

  The "Minimal Corresponding Source" for a Combined Work means the
Corresponding Source for the Combined Work, excluding any source code
for portions of the Combined Work that, considered in isolation, are
based on the Application, and not on the Linked Version.

  The "Corresponding Application Code" for a Combined Work means the
object code and/or source code for the Application, including any data
and utility programs needed for reproducing the Combined Work from the
Application, but excluding the System Libraries of the Combined Work.

  1. Exception to Section 3 of the GNU GPL.

  You may convey a covered work under sections 3 and 4 of this License
without being bound by section 3 of the GNU GPL.

  2. Conveying Modified Versions.

  If you modify a copy of the Library, and, in your modifications, a
facility refers to a function or data to be supplied by an Application
that uses the facility (other than as an argument passed when the
facility is invoked), then you may convey a copy of the modified
version:

   a) under this License, provided that you make a good faith effort to
   ensure that, in the event an Application does not supply the
   function or data, the facility still operates, and performs
   whatever part of its purpose remains meaningful, or

   b) under the GNU GPL, with none of the additional permissions of
   this License applicable to that copy.

  3. Object Code Incorporating Material from Library Header Files.

  The object code form of an Application may incorporate material from
a header file that is part of the Library.  You may convey such object
code under terms of your choice, provided that, if the incorporated
material is not limited to numerical parameters, data structure
layouts and accessors, or small macros, inline functions and templates
(ten or fewer lines in length), you do both of the following:

   a) Give prominent notice with each copy of the object code that the
   Library is used in it and that the Library and its use are
   covered by this License.

   b) Accompany the object code with a copy of the GNU GPL and this license
   document.

  4. Combined Works.

  You may convey a Combined Work under terms of your choice that,
taken together, effectively do not restrict modification of the
portions of the Library contained in the Combined Work and reverse
engineering for debugging such modifications, if you also do each of
the following:

   a) Give prominent notice with each copy of the Combined Work that
   the Library is used in it and that the Library and its use are
   covered by this License.

   b) Accompany the Combined Work with a copy of the GNU GPL and this license
   document.

   c) For a Combined Work that displays copyright notices during
   execution, include the copyright notice for the Library among
   these notices, as well as a reference directing the user to the
   copies of the GNU GPL and this license document.

   d) Do one of the following:

       0) Convey the Minimal Corresponding Source under the terms of this
       License, and the Corresponding Application Code in a form
       suitable for, and under terms that permit, the user to
       recombine or relink the Application with a modified version of
       the Linked Version to produce a modified Combined Work, in the
       manner specified by section 6 of the GNU GPL for conveying
       Corresponding Source.

       1) Use a suitable shared library mechanism for linking with the
       Library.  A suitable mechanism is one that (a) uses at run time
       a copy of the Library already present on the user's computer
       system, and (b) will operate properly with a modified version
       of the Library that is interface-compatible with the Linked
       Version.

   e) Provide Installation Information, but only if you would otherwise
   be required to provide such information under section 6 of the
   GNU GPL, and only to the extent that such information is
   necessary to install and execute a modified version of the
   Combined Work produced by recombining or relinking the
   Application with a modified version of the Linked Version. (If
   you use option 4d0, the Installation Information must accompany
   the Minimal Corresponding Source and Corresponding Application
   Code. If you use option 4d1, you must provide the Installation
   Information in the manner specified by section 6 of the GNU GPL
   for conveying Corresponding Source.)

  5. Combined Libraries.

  You may place library facilities that are a work based on the
Library side by side in a single library together with other library
facilities that are not Applications and are not covered by this
License, and convey such a combined library under terms of your
choice, if you do both of the following:

   a) Accompany the combined library with a copy of the same work based
   on the Library, uncombined with any other library facilities,
   conveyed under the terms of this License.

   b) Give prominent notice with the combined library that part of it
   is a work based on the Library, and explaining where to find the
   accompanying uncombined form of the same work.

  6. Revised Versions of the GNU Lesser General Public License.

  The Free Software Foundation may publish revised and/or new versions
of the GNU Lesser General Public License from time to time. Such new
versions will be similar in spirit to the present version, but may
differ in detail to address new problems or concerns.

  Each version is given a distinguishing version number. If the
Library as you received it specifies that a certain numbered version
of the GNU Lesser General Public License "or any later version"
applies to it, you have the option of following the terms and
conditions either of that published version or of any later version
published by the Free Software Foundation. If the Library as you
received it does not specify a version number of the GNU Lesser
General Public License, you may choose any version of the GNU Lesser
General Public License ever published by the Free Software Foundation.

  If the Library as you received it specifies that a proxy can decide
whether future versions of the GNU Lesser General Public License shall
apply, that proxy's public statement of acceptance of any version is
permanent authorization for you to choose that version for the
Library.
