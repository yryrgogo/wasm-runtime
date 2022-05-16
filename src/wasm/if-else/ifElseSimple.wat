(module
  (import "console" "log" (func $log (param i64)))
  (import "console" "log" (func $log2 (param f64)))
  (func (export "ifElseSimple")
    (param $value_1 i32)
    local.get $value_1
    (if
      (then
        i64.const 1
        call $log
      )
      (else
        i64.const 0
        call $log
      )
    )
  )
  (func
    (param $value_2 f64)
    (result f64)
    f64.const 0
    call $log2
    local.get $value_2
  )
)
