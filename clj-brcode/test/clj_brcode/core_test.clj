(ns clj-brcode.core-test
  (:require [clojure.test :refer :all]
            [clj-brcode.core :refer :all]))

(def code "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38")
(def edn {:payload-version 1, :initiation-methos nil, :merchant-information [{:id 26, :info [{:id 0, :info "BR.GOV.BCB.PIX"}, {:id 1, :info "123e4567-e12b-12d1-a456-426655440000"}]}, {:id 27, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123456789"}]}], :merchant-category-code 0, :merchant-name "NOME DO RECEBEDOR", :merchant-city "BRASILIA", :postal-code "70074900", :currency "986", :amount 123.45, :country-code "BR", :field-template [{:reference-label "RP12345678-2019"}], :crc1610 "AD38", :templates [{:id 80, :info [{:id 0, :info "BR.COM.OUTRO"}, {:id 1, :info "0123.ABCD.3456.WXYZ"}]}]})

(deftest a-test
  (testing "conform rust result to clojure"
    (is (= (brcode-from-str code) edn))))
