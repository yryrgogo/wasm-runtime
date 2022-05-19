(module
  (func $easy_return (export "easy_return") (result i32)
    i32.const 1
    (if
      (then
        i32.const 0
        return
      )
    )
    i32.const 1
  )
)
