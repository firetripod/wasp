# WASP 🐝
a LISP programming language for extremely performant and concise web assembly modules

**warning:** this compiler is very alpha and error messages aren't the best, but it works and language is simple!

```clojure
; main.w
(extern console_log [message])
(defn main "main" []
  (console_log "Hello World!")
)
```
```console
wasp build
```


# Features
* [x] immutable c-strings, memory manipulation, global variables, imported functions
* [x] optional standard library runtime
* [x] functions with inline web assembly
* [ ] easy project dependency management
* [ ] self hosting
* [ ] macros
* [ ] code pruning and inlining
* [ ] test framework support
* [ ] type optional
* [ ] eval
* [ ] repl

# Quickstart

Wasp depends on `git` and `rust`. Make sure you have them installed before beginning.

```console
cargo install wasp
wasp init myproject
cd myproject
wasp build
```
At this point we will have a web assembly module with a single exported main function and nothing else

```
wasp add std git@github.com:wasplang/std.git
wasp build
```
At this point we will have a web assembly module that has alot more access to the [standard libraries](https://github.com/wasplang/std) functions and a default.

# Drawing

Using [wasm-module](https://github.com/richardanaya/wasm-module) we can easily draw something to screen.

```clojure
(extern global_getWindow [])
(extern Window_get_document [window])
(extern Document_querySelector [document query])
(extern HTMLCanvasElement_getContext [element context])
(extern CanvasRenderingContext2D_fillRect [canvas x y w h])

(defn main "main" []
  (let [
      window (global_getWindow)
      document (Window_get_document window)
      canvas (Document_querySelector document "#screen")
      ctx (HTMLCanvasElement_getContext canvas "2d")
    ]
      (CanvasRenderingContext2D_fillRect ctx 0 0 50 50)
    )
)
```

See it working [here](https://wasplang.github.io/wasp/examples/canvas/index.html)

# Mutable Global Data

It's often important for a web assembly modules to have some sort of global data that can be changed.  For instance in a game we might have a high score.

```clojure
(def high_score (data 0) )

(defn run_my_game
  ...
  (mem32 high_score (+ (mem32 high_score) 100)  
  ...
)
```

# Project Management
**warning: this may change but it works**
Code dependencies are kept in a special folder called `vendor` which is populated by specific checkouts of git repositories.

For example a `project.wasp` containing:

```
bar git@github.com:richardanaya/bar.git@specific-bar
```

would result in these commands (roughly)

```
mkdir vendor
git clone git@github.com:richardanaya/bar.git@specific-bar vendor/bar
```

when `wasp build` is called

Now, when wasp compiles your code, it does a few things.

* In order specified by your `project.wasp`, one folder at a time all files ending in .w are loaded from each `vendor/<dependency-name>` and its subfolders.
* all files in the current directory and sub directories not in `vendor` are loaded
* then everything is compiled in order

Please try to use non conflicting names in meantime while this is fleshed out for 0.2.0

# Advanced
When necessary, low level web assembly can be directly inlined
```clojure
(defn-wasm memswap [i32] [i32] ; 1 input, 1 output
  [i32]  ; int tmp = 0;
  ; tmp = a
  LOCAL_GET   0
  LOCAL_SET   2
  ; a = b
  LOCAL_GET   1
  LOCAL_SET   0
  ; b = tmp
  LOCAL_GET   2
  LOCAL_SET   1
  END
)

(defn main "main" []
  ...
  (memswap 10, 20)
)
```

# Technical Details
## Types
* **integer** - a 32-bit integer (e.g `-1`, `0`, `42`)
* **string** - a 32-bit pointer to a location in memory of the start of of a c-string (e.g. `"hello world!"`)
* **symbol** - a 32-bit pointer to a location in memory of the start of of a c-string (e.g. `":hello_world"`)
* **bool** - a 32-bit number representing boolean values. True is 1, false is 0. (e.g. `true` `false`)
* **empty** - a 32-bit pointer to an empty value (e.g. `()`)
* **data** - a global only type this is a a 32-bit pointer to sequence of 32-bit values in memory (e.g. `(data 1 true :hey (data :more-data ())`). Use this for embedding raw data into your application memory on startup.

## Functions
* **(function-name ...)** - call a function with arguments
* **(mem x:integer)** - get 8-bit value from memory location x
* **(mem x:integer y)** - set 8-bit value at memory location x to value y
* **(mem32 x:integer)** - get 32-bit value from memory location x
* **(mem32 x:integer y)** - set 32-bit value at memory location x to value y
* **(if x y)** - if x is true return expression y otherwise return empty
* **(if x y z)** - if x is true return expression y otherwise return expression z
* **(do ... )** - executes a list of expressions and returns the value of the last. useful for multiple statement parameters.
* **(let [x0:identifier y0:expression x1:identifier y1:expression ... ] ... )** -  bind pairs of values to identifiers. Then run a sequence of expressions that can use those values by their identifier. Returns the value of the last expression in sequence. bindings specified in let shadow those at higher scopes.
* **(loop ... x )** - evaluate a list of expressions again and again
* **(break)** - break out of current loop and return an empty value
* **(continue)** - restart execution at start of loop
* **(+ ...)** - sums a list of values and returns result
* **(- ...)** - subtracts a list of values and returns result
* **(\* ...)** - multiplies a list of values and returns result
* **(/ ...)** - divides a list of values and returns result
* **(% ...)** - modulos a list of values and returns result
* **(== x y)** - returns true if values are equal, false if otherwise
* **(!= x )** - returns true if values are not equal, false if otherwise
* **(< x y)** -  returns true if x is less than y, false if otherwise
* **(> x y)** - returns true if x is greater than y, false if otherwise
* **(<= x y)** - returns true if x is less than or equal y, false if otherwise
* **(>= x y)** - returns true if x is greater than or equal y, false if otherwise
* **(&& x y)** - returns true if x and y are greater than zero, false if otherwise
* **(|| x y)** - returns true if x or y are greater than zero, false if otherwise
* **(& x y)** - returns bitwise and of x and y
* **(| x y)** - returns bitwise or of x and y
* **(! x )** - returns true if zero and false if not zero
* **(^ x )** - bitwise exclusive or of x
* **(~ x )** - bitwise complement of x
* **(<< x y)** - shift x left by y bits
* **(>> x y)** - shift x right by y bits

## Why so few functions?
Wasp prefers to keep as little in the core functionality as possible, letting the [standard library](https://github.com/wasplang/std) evolve faster and more independent community driven manner. This project currently follows a principle that if a feature can be implemented with our primitive functions, don't include it in the core compiled language and let the standard library implement it. Also that no heap based concepts be added to the core language.

## Notes
<p align="center">
<img src="static_heap.svg" width="400">
</p>

* all functions (including extern functions) return a value, if no obvious return, it is an empty value `()`
* Web assembly global 0 is initialized to the end of the static data section (which might also be the start of a heap for a memory allocator). This value is immutable. 
* Web assembly global lobal 1 also is initialized to the end of the static data section. This value is mutable and might be used to represent the end of your heap. Check out the [simple allocator example](https://github.com/richardanaya/wasp/blob/master/examples/malloc/main.w).
* Literal strings create initialize data of a c-string at the front of your memory, and can be passed around as pointers to the very start in memory to your text. A \0 is automatically added at compile time, letting you easily have a marker to denote the end of your text.
