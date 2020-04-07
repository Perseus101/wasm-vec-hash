#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn standard_hash() {
        let message = vec![10, 20, 30, 40, 50];
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);

        assert_eq!(3270947794393102004, hasher.finish());
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn wasm_hash() {
        let message = vec![10, 20, 30, 40, 50];
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);

        assert_eq!(3270947794393102004, hasher.finish());
    }
}
