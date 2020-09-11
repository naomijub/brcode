(ns clj-brcode.core
  (:import jnr.ffi.LibraryLoader))

(def mem-brcode
    (let [lib-brcode (-> (gen-interface :name "LibC" :methods [[edn_to_brcode [String] String]])
                         LibraryLoader/create
                         (.load "brcode"))]
      lib-brcode))

(defn brcode-from-str [s]          
    (-> mem-brcode (.edn_to_brcode s) read-string))
  
