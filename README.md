# wrap [![Build Status](https://travis-ci.org/jaemk/wrap.svg?branch=master)](https://travis-ci.org/jaemk/wrap) [![crates.io](https://img.shields.io/crates/v/wrap.svg)](https://crates.io/crates/wrap) [![docs](https://docs.rs/wrap/badge.svg)](https://docs.rs/wrap)

> generalized rust wrapping macro

Macros for defining generalized `wrappers` and applying them to arbitrary functions. See `examples` for more advanced `wrappers`.

## Simple Usage


```rust,ignore
#[macro_use] extern crate wrap;


def_wrapper!{log1 =
    before = (fn_args) >> {
        println!("* [log-1] >> before everything! fn_args: {:?}", fn_args);
    };
    after  = (wrapped_result) >> {
        println!("* [log-1] >> after everything! wrapped_result: {:?}", wrapped_result);
    };
}


def_wrapper!{log2 =
    before = (fn_args) >> {
        println!("* [log-2] >> before everything! fn_args: {:?}", fn_args);
    };
    after  = (wrapped_result) >> {
        println!("* [log-2] >> after everything! wrapped_result: {:?}", wrapped_result);
    };
}


wrap_with!{log1 >>
fn greet_logged_inner(name: &str) -> String = {
    format!("How are you, {}?", name)
}}


wrap_with!{log2 >>
fn greet_logged(name: &str) -> String = {
    format!("Hello! {}", greet_logged_inner(name))
}}


pub fn main() {
    println!("{}", greet_logged("james"));
}
```

