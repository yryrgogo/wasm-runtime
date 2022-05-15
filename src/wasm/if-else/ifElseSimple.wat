(module
  (import "console" "log" (func $log (param i32)))
  (func
    i32.const 0
    (if
      (then
        i32.const 1
        call $log
      )
      (else
        i32.const 0
        call $log
      )
    )
  )

  (start 1)
)
