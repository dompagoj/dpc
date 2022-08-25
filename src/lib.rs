#![allow(dead_code)]
#![allow(unused)]

mod commands;
mod io;
mod utils;

#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;
use napi::*;

use crate::commands::{CommandRunner, FindConstFuncs};
use std::fmt::Debug;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, num};

#[derive(Clone, Debug)]
enum JsAny {
    Null,
    Number(f64),
    Str(String),
    Bool(bool),
}

impl ToNapiValue for JsAny {
    unsafe fn to_napi_value(env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
        dbg!(&val);

        let val = match val {
            JsAny::Null => {}
            JsAny::Number(num) => {}
            JsAny::Str(str) => {}
            JsAny::Bool(b) => {}
        };
        todo!()
    }
}

impl FromNapiValue for JsAny {
    unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
        if (napi_val.is_null()) {
            return Ok(JsAny::Null);
        }

        let num_result = f64::from_napi_value(env, napi_val);
        if num_result.is_ok() {
            return Ok(JsAny::Number(num_result.unwrap()));
        }

        dbg!("Num result failed");

        let bool_result = bool::from_napi_value(env, napi_val);

        if (bool_result.is_ok()) {
            return Ok(JsAny::Bool(bool_result.unwrap()));
        }

        dbg!("Bool result failed!");

        let str_result = String::from_napi_value(env, napi_val);
        if str_result.is_ok() {
            return Ok(JsAny::Str(str_result.unwrap()));
        }

        dbg!("String result failed!");

        return Err(napi::Error::from_reason("Failed to serialize JsAny"));
    }
}

#[napi(ts_return_type = "Promise<void>")]
fn find_const_functions<T: Fn(Vec<String>) -> Result<Promise<Vec<EvaluatedConstFunction>>>>(
    env: Env,
    module_path: String,
    cb: T,
) -> Result<JsObject> {
    let const_funcs = FindConstFuncs { module_path }.execute().unwrap();

    let promise = cb(const_funcs).unwrap();

    env.execute_tokio_future(async move { Ok(promise.await.unwrap()) }, |_, data| {
        dbg!(&data);
        Ok(data)
    })
}

#[napi]
fn compile(path: String) {}

#[napi(object)]
#[derive(Debug)]
struct EvaluatedConstFunction {
    pub name: String,
    pub value: JsAny,
}

#[napi]
fn replace_const_functions(funcs: Vec<EvaluatedConstFunction>) -> napi::Result<()> {
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------
//      Old way of doing things with napi-rs, a bit more flexible when types are unknown, could be usefull
// ---------------------------------------------------------------------------------------------------------------------
// #[js_function(1)]
// fn test(ctx: CallContext) -> Result<JsString> {
//     let a = ctx.get::<JsString>(0).unwrap();

//     Ok(a)
// }

// #[module_exports]
// fn init(mut exports: JsObject) -> Result<()> {
//     exports.create_named_method("test", test)?;
//     Ok(())
// }
