macro_rules! bind_function {
    (($($args:ty),*) -> $ret:ty, $fn:ident) => {

    };
}

#[macro_export]
macro_rules! deque {
    [$($elems:expr),*] => {
        {
            use std::collections::VecDeque;
            let mut ret = VecDeque::new();
            $(
                ret.push_back($elems);
            )*
            ret
        }
    };
}
