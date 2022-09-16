(module
  (func (export "block") (result i32)
    (local $i i32)
    (local $sum i32)
    (local.set $sum (i32.const 0))
    (local.set $i (i32.const 0))
    (block
      (local.set $i (i32.add (local.get $i) (i32.const 3)))
      (local.set $sum (i32.add (local.get $i) (i32.const 11)))
    )
    (local.get $sum)
  )
)
