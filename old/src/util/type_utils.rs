pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
