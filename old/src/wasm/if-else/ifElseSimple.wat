;; 外部から import した関数を利用する VM の処理の書き方がわからない
(module
  (import "console" "log" (func $log (param i64)))
  (import "console" "log" (func $log2 (param i32)))
  (func (export "ifElseSimple")
    (param $value_1 i32)
    local.get $value_1
    (if
      (then
        i64.const 1
        call $log
      )
      (else
        i32.const 0
        call $log2
      )
    )
  )
)
