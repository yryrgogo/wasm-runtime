(module
  (func $add (param $p1 i32) (param $p2 i32) (result i32)
    (i32.add (local.get $p1) (local.get $p2))
  )

  (func (export "increment") (param $p i32) (result i32)
    (call $add (local.get $p) (i32.const 1))
  )
)
