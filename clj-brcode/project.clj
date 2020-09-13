(defproject clj-brcode "1.0.0-SNAPSHOT"
  :description "Clojure wrapper of Rust's `brcode` parser and emitter"
  :url "http:https://github.com/naomijub/brcode"
  :license {:name "LGPL-3.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.10.1"]
                 [com.github.jnr/jnr-ffi "2.1.16"]]
  :main ^:skip-aot clj-brcode.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
