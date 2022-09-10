(module
  (func (export "sub") (param $p1 i32) (param $p2 i32) (result i32)
    (i32.sub (local.get $p1) (local.get $p2))
  )
)
