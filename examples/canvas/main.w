(extern global_getWindow [])
(extern Window_get_document [window])
(extern Document_querySelector [document query])
(extern HTMLCanvasElement_getContext [element context])
(extern CanvasRenderingContext2D_fillRect [canvas x y w h])

(defn main "main" []
(let [window (global_getWindow)
      document (Window_get_document window)
      canvas (Document_querySelector document "#screen")
      ctx (HTMLCanvasElement_getContext canvas "2d")]
  (CanvasRenderingContext2D_fillRect ctx 0 0 50 50)))
