(module
  ;; (table $tbl (export "tbl") 1 anyfunc)
  (global $i (mut i32)(i32.const -1))
  (global $j (mut i32)(i32.const -1))

  (func $increment (export "increment") (result i32)
    (global.set $i (i32.add (global.get $i) (i32.const 1)))
    global.get $i
  )

  ;; (elem (i32.const 0) $increment)
)
