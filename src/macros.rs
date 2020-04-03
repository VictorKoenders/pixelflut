#[macro_export]
macro_rules! callback {
    (|$($e:ident),*| $inner:block) => {
        {
            $(
                let $e = $e.clone();
            )*
            move || $inner
        }
    };
}
