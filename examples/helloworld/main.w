(extern console_log [message])
(defn b [f]
  (f "hello world123!"))

(defn main "main" [msg]
  (b console_log))
