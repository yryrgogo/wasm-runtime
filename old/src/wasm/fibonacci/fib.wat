(module
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (export "fib" (func $fib))
 (func $fib (param $p0 i32) (result i32)
  (local $l0 i32)
  (local $l1 i32)
  (local $l2 i32)
  (if
   (i32.ge_u
    (local.get $p0)
    (i32.const 2)
   )
   (block
    (local.set $l0
     (i32.add
      (local.get $p0)
      (i32.const -1)
     )
    )
    (local.set $p0
     (i32.const 1)
    )
    (loop $label$2
     (local.set $p0
      (i32.add
       (local.tee $l2
        (local.get $p0)
       )
       (local.get $l1)
      )
     )
     (local.set $l1
      (local.get $l2)
     )
     (br_if $label$2
      (local.tee $l0
       (i32.add
        (local.get $l0)
        (i32.const -1)
       )
      )
     )
    )
   )
  )
  (local.get $p0)
 )
)
