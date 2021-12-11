#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let h_set = HashSet::new();
            $(
                h_set.insert($x);
            )*
            h_set
        }
    };
}
