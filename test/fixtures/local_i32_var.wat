(module
  (func (export "local_i32_var") (result i32)
    (local $var i32)
		(local.set $var (i32.const 55))
		(local.get $var)
	)
)
