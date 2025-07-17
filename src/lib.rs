use std::thread;

use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

#[napi]
pub fn test(tsfn: ThreadsafeFunction<()>) {
    thread::spawn(|| {
        let _f = tsfn;
        // drop(_f);
        panic!("tsfn is not dropped")
    });
}
