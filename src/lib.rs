
#[macro_export]
macro_rules! def_wrapper {
    ($name:ident = before = ($fn_args:ident) >> $before:expr ; after = ($wrapped_res:ident) >> $after:expr;) => {
        macro_rules! $name {
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
            $wrapper!( _before_block_res : () ;  ($(&$arg),*) => $ret = $body)
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


