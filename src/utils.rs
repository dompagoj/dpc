use napi::Status;

pub fn anyhow_result_to_napi_result<T>(res: anyhow::Result<T>) -> napi::Result<T> {
    match res {
        anyhow::Result::Ok(t) => Ok(t),
        anyhow::Result::Err(err) => Err(napi::Error::new(Status::Unknown, err.to_string())),
    }
}

macro_rules! cast_enum {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

pub(crate) use cast_enum;
