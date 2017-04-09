#[macro_use] extern crate wrap;
#[macro_use] extern crate lazy_static;

use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;


/// Setup a fake database
lazy_static! {
    static ref DB: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert("a".into(), "A".into());
        Mutex::new(m)
    };
}

/// Setup a fake database connection thing
#[derive(Debug)]
struct Conn<'a>(MutexGuard<'a, HashMap<String, String>>);
impl<'a> Conn<'a> {
    fn query(&self, key: &str) -> Option<String> {
        self.0.get(key).map(|s| s.clone())
    }
    fn insert(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }
}


/// Define a wrapper that provides a `Conn` instance to wrapped functions
def_wrapper!{with_conn =
    before = (_fn_args) >> {
        println!("* in `with_conn` wrapper");
        // you can return early from `before`. this will return before
        // the wrapped function is executed.
        //return "hijacked!".to_string();

        Conn(DB.lock().unwrap())
    };
    after  = (wrapped_result) >> {};
}


/// Modifies the return result of a wrapped function that returns an Option<String>
def_wrapper!{spooky_action =
    before = (_fn_args) >> {};
    after  = (wrapped_result) >> {
        println!("* in `spooky_action` wrapper");
        wrapped_result = wrapped_result.map(|mut s| {
            s.push_str(" !! spooky !!");
            s
        });
    };
}


def_wrapper!{log =
    before = (fn_args) >> {
        println!("* [log] >> before everything! fn_args: {:?}", fn_args);
    };
    after  = (wrapped_result) >> {
        println!("* [log] >> after everything! wrapped_result: {:?}", wrapped_result);
    };
}


/// Generate a function wrapped with `with_conn` to provide a binding
/// `conn` of type `Conn` to the function body
wrap_with!{with_conn -> conn: Conn; >>
fn get_key(key: &str) -> Option<String> = {
    let v: Option<String> = conn.query(key);
    v
}}


/// Generate a function wrapped with `spooky_action` that modifies
/// this function body's return value
wrap_with!{spooky_action -> _nil: (); >>
fn get_key_spooky(key: &str) -> Option<String> = {
    get_key(key)
}}


/// Wrap get-spooky with a logging wrapper.
/// Note, that the wrapper provided value ident and type
/// can be excluded if you don't want/need access to them
/// inside of your function
wrap_with!{log >>
fn get_key_spooky_logged(key: &str) -> Option<String> = {
    get_key_spooky(key)
}}


wrap_with!{with_conn -> mut conn: Conn; >>
fn set_key(key: String, value: String) -> () = {
    conn.insert(key, value);
}}


pub fn main() {
    println!("setting key...");
    set_key("james".into(), "JAMES".into());

    println!("\ngetting key...");
    let v: Option<String> = get_key("a");
    println!("get_key(\"a\") -> {:?}", v);

    println!("\ngetting key [spooky]...");
    let v = get_key_spooky("a");
    println!("get_key_spooky(\"a\") -> {:?}", v);

    println!("\ngetting key [spooky & logged]...");
    let v = get_key_spooky_logged("a");
    println!("get_key_spooky_logged(\"a\") -> {:?}", v);
}
