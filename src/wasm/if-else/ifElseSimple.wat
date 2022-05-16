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
        f64.const 0
        call $log2
      )
    )
  )
)
