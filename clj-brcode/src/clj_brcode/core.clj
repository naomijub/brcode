(ns clj-brcode.core
  (:import jnr.ffi.LibraryLoader)
  (:gen-class))

(def mem-brcode
    (let [lib-brcode (-> (gen-interface :name "LibC" :methods [[edn_from_brcode [String] String] [edn_to_brcode [String] String]])
                         LibraryLoader/create
                         (.load "brcode"))]
      lib-brcode))

(defn brcode->edn [s]          
    (-> mem-brcode (.edn_from_brcode s) read-string))

(defn edn->brcode [edn]
  (-> mem-brcode (.edn_to_brcode (pr-str edn))))
  
