#[macro_use] extern crate wrap;
#[macro_use] extern crate lazy_static;

use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;


lazy_static! {
    static ref DB: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert("a".into(), "A".into());
        Mutex::new(m)
    };
}


#[derive(Debug)]
struct Conn<'a>(MutexGuard<'a, HashMap<String, String>>);
impl<'a> Conn<'a> {
    fn query(&self, key: &str) -> Option<String> {
        self.0.get(key).map(|s| s.clone())
    }
}


def_wrapper!{with_conn =
    before = fn_args >> {
        println!("log before -> pre_args: {:?}", fn_args);
        //return "hijacked!".to_string();
        Conn(DB.lock().unwrap())
    };
    after  = wrapped_result >> {
        println!("log after -> wrapped_result {:?}", wrapped_result);
        wrapped_result = wrapped_result.map(|mut s| {
            s.push_str(" !! spooky !!");
            s
        });
        println!("modifying wrapped_result! -> {:?}", wrapped_result);
    };
}


wrap_with!{with_conn -> conn: Conn; >>
fn get_key(key: &str) -> Option<String> = {
    println!(" in wrapped! ->> {:?}", conn);
    let v: Option<String> = conn.query(key);
    println!("found: {:?}", v);
    v
}}

pub fn main() {
    let v: Option<String> = get_key("a");
    println!("{:?}", v);
}
