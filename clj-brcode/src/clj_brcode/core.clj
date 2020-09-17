(ns clj-brcode.core
  (:import jnr.ffi.LibraryLoader)
  (:gen-class))

(def mem-brcode
    (let [lib-brcode (-> (gen-interface :name "LibC" :methods
                           [[edn_from_brcode [String] String] 
                            [edn_to_brcode [String] String]
                            [json_from_brcode [String] String]
                            [json_to_brcode [String] String]
                            [crc16_ccitt_from_message [String] String]])
                         LibraryLoader/create
                         (.load "brcode"))]
      lib-brcode))

(defn brcode->edn [s]          
    (-> mem-brcode (.edn_from_brcode s) read-string))

(defn edn->brcode [edn]
  (-> mem-brcode (.edn_to_brcode (pr-str edn))))

(defn brcode->json [s]
  (-> mem-brcode (.json_from_brcode s)))

(defn json->brcode [json]
  (-> mem-brcode (.json_to_brcode json)))

(defn crc16-ccitt [message]
  (-> mem-brcode (.crc16_ccitt_from_message message)))
  
