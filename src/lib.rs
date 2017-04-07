/*!
Macros for defining function `wrappers` and `wrapped` functions.


# Usage:

`wrap` provides two macros, `def_wrapper!` and `wrap_with!`.
`def_wrapper!` lets you define `before` and `after` actions to wrap any given function.
`wrappers` can then be applied to arbitrary functions using `wrap_with!` which takes a `wrapper` name and
function definition and expands to a wrapped form.

`before` blocks can be used to evaluate the wrapped function's arguments, create and pass on contextual info to the wrapped function,
and short-circuit to return before evaluation of the wrapped function. `before` blocks' access to the wrapped function arguments
is limited to a tuple of references, `(&arg1, &arg2, ...)`. `before` blocks cannot own or mutate function arguments as that would
require cloning all arguments and defining `wrap_with!` functions to take `mut` arguments.

`after` blocks can be used to mutate the wrapped function's result and can access the `before` block's value. `after` blocks cannot
access the wrapped function's arguments as the wrapped function is expected to consume them.

See examples for more complex wrappers.


```rust
#[macro_use] extern crate wrap;


def_wrapper!{log_wrap =
    before = (fn_args) >> {
        println!("* [log] >> before everything! fn_args: {:?}", fn_args);
    };
    after  = (wrapped_result) >> {
        println!("* [log] >> after everything! wrapped_result: {:?}", wrapped_result);
    };
}


wrap_with!{log_wrap >>
fn greet(name: &str) -> String = {
    format!("Hello, {}!", name)
}}


pub fn main() {
    greet("bean");
}
```

*/

#[macro_export]
macro_rules! def_wrapper {
    ($name:ident = before = ($fn_args:ident) >> $before:expr ; after = ($wrapped_res:ident) >> $after:expr;) => {
        macro_rules! $name {
            (_ : _; $args:expr => $wrapped_body_ret:ty = $wrapped_body:expr) => {
                {
                    let mut _before  = {
                        let $fn_args = $args;
                        $before
                    };
                    let mut $wrapped_res: $wrapped_body_ret = (|| $wrapped_body)();
                    let _ = $after;
                    $wrapped_res
                }
            };
            ($before_block_res:ident : $before_block_ty:ty ; $args:expr => $wrapped_body_ret:ty = $wrapped_body:expr) => {
                {

                    let mut $before_block_res: $before_block_ty = {
                        // assign wrapped function's args (refs to args collected in a tuple)
                        // to our `fn_args` ident, to make available to the `before` block
                        let $fn_args = $args;

                        // eval the `before` block and assign to the `before_block_res` ident supplied by `wrap_with!`
                        $before
                    };

                    // eval our wrapped function, providing `&mut before_block_res` to the `wrapped_body`,
                    // assigning the result to our ``wrapped_res` ident to make available to our `after` block and
                    // also keep `before_block_res` in scope for the `after` block
                    let mut $wrapped_res: $wrapped_body_ret = (|$before_block_res: &mut $before_block_ty| $wrapped_body)(&mut $before_block_res);

                    // eval the `after` block. Can access and mutate both `wrapped_res` and `before_block_res` if necessary.
                    // cannot access `fn_args` because they are likely consumed by the wrapped function.
                    let _ = $after;

                    $wrapped_res
                }
            };

            // same as above, but accept and pass along `mut` keyword
            (mut $before_block_res:ident : $before_block_ty:ty ; $args:expr => $wrapped_body_ret:ty = $wrapped_body:expr) => {
                {
                    let mut $before_block_res: $before_block_ty = {
                        let $fn_args = $args;
                        $before
                    };
                    let mut $wrapped_res: $wrapped_body_ret = (|mut $before_block_res: &mut $before_block_ty| $wrapped_body)(&mut $before_block_res);
                    let _ = $after;
                    $wrapped_res
                }
            };
        }
    }
}


#[macro_export]
macro_rules! wrap_with {
    ($wrapper:ident >> fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        #[allow(unused_mut, unused_parens)]
        pub fn $name($($arg : $argtype),*) -> $ret {
            $wrapper!( _ : _ ; ($(&$arg),*) => $ret = $body)
        }
    };
    ($wrapper:ident -> $before_block_res:ident : $before_block_ty:ty; >> fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        #[allow(unused_mut, unused_parens)]
        pub fn $name($($arg : $argtype),*) -> $ret {
            $wrapper!( $before_block_res : $before_block_ty ;  ($(&$arg),*) => $ret = $body)
        }
    };
    // pass along `mut ident`
    ($wrapper:ident -> mut $before_block_res:ident : $before_block_ty:ty; >> fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        #[allow(unused_mut, unused_parens)]
        pub fn $name($($arg : $argtype),*) -> $ret {
            $wrapper!( mut $before_block_res : $before_block_ty ;  ($(&$arg),*) => $ret = $body)
        }
    };
}


