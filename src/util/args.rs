use std::env;

pub fn get_args() -> (String, Vec<i32>) {
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
		.map(|x| x.parse::<i32>().unwrap())
		.collect::<Vec<i32>>();

	(path, num_args)
}
