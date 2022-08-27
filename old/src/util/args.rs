use std::env;

use crate::module::number::Number;

pub fn get_args() -> (String, Vec<Number>) {
	let dir_path = env::current_dir().unwrap();
	let args: Vec<String> = env::args().collect();
	let wasm_module_path = args.get(1).unwrap_or_else(|| {
		panic!("wasm モジュールへのパスを渡してください。（ルートディレクトリからの相対パス）")
	});
	if !wasm_module_path.contains(".wasm") {
		panic!("wasm モジュールへのパスを渡してください。（ルートディレクトリからの相対パス）");
	}
	let path = format!("{}/{}", dir_path.to_string_lossy(), wasm_module_path);

	let num_args = args[2..]
		.iter()
		.map(|x| -> Number {
			let values: Vec<&str> = x.split(".").collect();
			let result =
		// 小数の場合
		if values.len() > 1 {
			// 小数点以下の指定が6桁までなら f32 とする
			// 7桁目で1.1 + 2.2 で7桁目に揺れがあったのでこうした
			if values.get(1).unwrap().len() <= 6 {
				let num = x.parse::<f32>().unwrap();
				// TODO: f32 を u32 に変換する
				Number::Float32(num as u32)
			} else {
				let num = x.parse::<f64>().unwrap();
				// TODO: f64 を u64 に変換する
				Number::Float64(num as u64)
			}
		} else if x.starts_with("-") {
			let num = x.parse::<i64>().unwrap();
			if num < std::i32::MIN as i64 {
				Number::Int64(num)
			} else {
				Number::Int32(num as i32)
			}
		} else {
			let num = x.parse::<i64>().unwrap();
			if num > std::i32::MAX as i64 {
				Number::Int64(num)
			} else {
				Number::Int32(num as i32)
			}
		};

			result
		})
		.collect::<Vec<Number>>();

	(path, num_args)
}
