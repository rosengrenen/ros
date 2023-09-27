#[macro_export]
macro_rules! vec {
    ($alloc:expr) => (
        $crate::vec::Vec::new($alloc)
    );
    ($alloc:expr; $elem:expr; $n:expr) => (
        $crate::vec::Vec::from_elem($elem, $n, $alloc)
    );
    ($alloc:expr; $($x:expr),+ $(,)?) => {{
        let mut vec = $crate::vec::Vec::new($alloc);
        $(
            vec.push($x).unwrap();
        )+
        vec
    }}
}
