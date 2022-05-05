use std::env;

pub fn get_module_path() -> String {
	let dir_path = env::current_dir().unwrap();
	let args: Vec<String> = env::args().collect();
	let wasm_module_path = args.get(1).unwrap_or_else(|| {
		panic!("wasm モジュールへのパスを渡してください。（ルートディレクトリからの相対パス）")
	});
	let path = format!("{}/{}", dir_path.to_string_lossy(), wasm_module_path);

	path
}
