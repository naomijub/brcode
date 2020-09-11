# clj-brcode

Clojure wrapper of `brcode` to parse and emit [PIX BR Code](https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf).

## Installation

Download from http://example.com/FIXME.

## Usage

FIXME: explanation

    $ java -jar clj-brcode-0.1.0-standalone.jar [args]

## Examples

```clojure
(def brcode "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38")

(def edn {:payload-version 1, :initiation-methos nil, :merchant-information [{:id 26, :info [{:id 0, :info "BR.GOV.BCB.PIX"}, {:id 1, :info "123e4567-e12b-12d1-a456-426655440000"}]}, {:id 27, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123456789"}]}], :merchant-category-code 0, :merchant-name "NOME DO RECEBEDOR", :merchant-city "BRASILIA", :postal-code "70074900", :currency "986", :amount 123.45, :country-code "BR", :field-template [{:reference-label "RP12345678-2019"}], :crc1610 "AD38", :templates [{:id 80, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123.ABCD.3456.WXYZ"}]}]})

(= (brcode-from-str brcode) edn)
```

## License

Copyright © 2020 FIXME

This program and the accompanying materials are made available under the
terms of the Eclipse Public License 2.0 which is available at
http://www.eclipse.org/legal/epl-2.0.

This Source Code may also be made available under the following Secondary
Licenses when the conditions for such availability set forth in the Eclipse
Public License, v. 2.0 are satisfied: GNU General Public License as published by
the Free Software Foundation, either version 2 of the License, or (at your
option) any later version, with the GNU Classpath Exception which is available
at https://www.gnu.org/software/classpath/license.html.
