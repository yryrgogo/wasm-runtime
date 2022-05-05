(module
  (func (export "twice")
    (param $value i32)
    (result i32)
    local.get $value
    i32.const 1
    i32.shl
	)
)
