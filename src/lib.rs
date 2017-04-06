
#[macro_export]
macro_rules! def_wrapper {
    ($name:ident = before = $fn_args:ident >> $before:expr ; after = $wrapped_res:ident >> $after:expr;) => {
        macro_rules! $name {
            ($args:expr; $before_block_res:ident : $before_block_ty:ty ; $wrapped_body:expr => $wrapped_body_ret:ty) => {
                {
                    // assign wrapped function's args (collected in a tuple)
                    // to our `fn_args` ident, to make available to `before` and `after` blocks.
                    let $fn_args = $args;

                    // eval `before` block and assign to the `before_block_res` ident supplied by `wrap_with!`
                    let $before_block_res: $before_block_ty = $before;

                    // eval our wrapped function, providing `before_block_res` to the `wrapped_body`,
                    // assigning the result to our ``wrapped_res` ident to make available to our `after` block.
                    let mut $wrapped_res: $wrapped_body_ret = (|$before_block_res: $before_block_ty| $wrapped_body)($before_block_res);

                    // eval `after` block. Can mutate the `wrapped_res` if necessary
                    let _ = $after;

                    $wrapped_res
                }
            };
        }
    }
}


#[macro_export]
macro_rules! wrap_with {
    ($wrapper:ident -> $before_block_res:ident : $before_block_ty:ty; >> fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        #[allow(unused_mut, unused_parens)]
        pub fn $name($($arg : $argtype),*) -> $ret {
            $wrapper!( ($($arg),*) ; $before_block_res : $before_block_ty ; $body => $ret)
        }
    }
}
