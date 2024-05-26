#[macro_export]
macro_rules! dict {
    ($($k:expr => $v:expr),*) => {{
        let mut m = BTreeMap::new();
        $(m.insert($k, $v);)*
        m
    }}
}
