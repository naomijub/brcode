(ns clj-brcode.core
  (:import jnr.ffi.LibraryLoader)
  (:gen-class))

(def mem-brcode
    (let [lib-brcode (-> (gen-interface :name "LibC" :methods [[edn_from_brcode [String] String]])
                         LibraryLoader/create
                         (.load "brcode"))]
      lib-brcode))

(defn brcode-from-str [s]          
    (-> mem-brcode (.edn_from_brcode s) read-string))
  
