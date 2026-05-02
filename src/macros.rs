#[macro_export]
macro_rules! map {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut data = ::std::collections::HashMap::new();
        $( data.insert($k, $v); )*
        data
    }};
}

#[macro_export]
macro_rules! try_or {
    ($expr:expr, $default:expr) => {
        match $expr {
            Ok(v) => v,
            Err(_) => $default,
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($v:expr) => {
        Ok($v)
    };
}

#[macro_export]
macro_rules! err {
    ($v:expr) => {
        Err($v)
    };
}

#[macro_export]
macro_rules! time_async {
    ($label:expr, $future:expr) => {{
        let start = std::time::Instant::now();
        let result = $future.await;
        println!("{}: {:?}", $label, start.elapsed());
        result
    }};
}

#[macro_export]
macro_rules! vec_from {
    ( $( $v:expr ),* $(,)? ) => {{
        let mut v = Vec::new();
        $( v.push($v); )*
        v
    }};
}

#[macro_export]
macro_rules! first {
    ($v:expr) => {
        $v.first()
    };
}

#[macro_export]
macro_rules! response {
    ($status:expr, $data:expr) => {{
        use serde_json::json;

        let obj = json!({
            "status": $status,
            "data": $data
        });

        crate::Json::new(obj)
    }};
}

#[macro_export]
macro_rules! define {
    ($name:ident, $value:expr) => {
        pub const $name: &str = $value;
    };

    ($name:ident, $value:expr, $t:ty) => {
        pub const $name: $t = $value;
    };
}