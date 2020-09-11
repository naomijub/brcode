(ns clj-brcode.core
  (:import jnr.ffi.LibraryLoader))

(defn brcode-from-str [s]
  (let [brcode (-> (gen-interface :name "LibC" :methods [[edn_to_brcode [String] String]])
                   LibraryLoader/create
                   (.load "brcode"))]
    (-> brcode (.edn_to_brcode s) read-string)))
  


