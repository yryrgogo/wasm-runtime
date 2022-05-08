(module
  (func (export "addFloat")
    (param $value_1 f32) (param $value_2 f32)
    (result f32)
    local.get $value_1
    local.get $value_2
    f32.add
	)
)

