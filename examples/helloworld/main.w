(extern console_log [message])
(defn do_it [f]
  (f "hello world123!"))

(defn main "main" [msg]
  (do_it console_log))
