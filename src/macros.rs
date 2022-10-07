#[macro_export]
macro_rules! count_tokens {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + super::count_tokens!($($xs)*));
}

/// If there are tokens, paste in a block otherwise paste else block.
#[macro_export]
macro_rules! if_token {
    (item($($_:item)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (type($($_:ty)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (expr($($_:expr)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (block($($_:block)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (pat($($_:pat)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (meta($($_:meta)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (ident($($_:ident)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (stmt($($_:stmt)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (path($($_:path)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (lit($($_:literal)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (lifetime($($_:lifetime)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (() => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (none() => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
    (none($($_:tt)+) => $trueblock:block $( else $elseblock:block)?) => {
        $($elseblock)?
    };
    ($($_:ident)?() => $trueblock:block $( else $elseblock:block)?) => {
        $($elseblock)?
    };
    (($($_:tt)+) => $trueblock:block $( else $elseblock:block)?) => {
        $trueblock
    };
}

#[macro_export]
macro_rules! callback {
    ($visibility:vis $name:ident($($paramname:ident: $paramtype:ty),*) $(-> $returntype:ty)?) => {
        $visibility struct $name<'callback> {
            pub callback: Option<Box<dyn FnOnce($($paramtype,)*) $(-> $returntype)? + 'callback>>,
        }

        #[allow(unused)]
        impl<'callback,F> From<F> for $name<'callback>
        where F: FnOnce($($paramtype,)*) $(-> $returntype)? + 'callback {
            fn from(once: F) -> Self {
                Self {
                    callback: Some(Box::new(once)),
                }
            }
        }

        #[allow(unused)]
        impl<'callback> $name<'callback> {

            #[inline(always)]
            pub fn new(callback: impl FnOnce($($paramtype,)*) $(-> $returntype)? + 'callback) -> Self {
                Self {
                    callback: Some(Box::new(callback)),
                }
            }

            #[inline(always)]
            pub fn invoke(mut self, $($paramname: $paramtype,)*) $(-> $returntype)? {
                if_token!(type($($returntype)?) => {
                    (
                        self.callback
                            .take()
                            .expect("Callback already consumed.")
                    )($($paramname,)*)
                } else {
                    (
                        self.callback
                            .take()
                            .expect("Callback already consumed.")
                    )($($paramname,)*);
                })
            }

            #[inline(always)]
            pub fn invoke_if(mut self, $($paramname: $paramtype,)*) $(-> Option<$returntype>)? {
                if_token!(tokens($($returntype)?) => {
                    self.callback
                        .take()
                        .map(|callback| callback($($paramname,)*))
                } else {
                    self.callback
                        .take()
                        .map(|callback| { callback($($paramname,)*); });
                })
            }
        }
    };
}

#[allow(unused)]
pub use callback;
#[allow(unused)]
pub use count_tokens;
#[allow(unused)]
pub use if_token;

#[cfg(test)]
mod tests {
    #[test]
    fn if_token_test() {
        macro_rules! if_token_test {
            ($branch:ident[$($token:tt)*]) => {
                if_token!(
                    $branch() => {
                        panic!("[if_token_test] failed for branch: not {}", stringify!($branch));
                    } else {
                        println!("[if_token_test] passed for branch: not {}", stringify!($branch));
                    }
                );
                if_token!(
                    $branch($($token)*) => {
                        println!("[if_token_test] passed for branch: {}", stringify!($branch));
                    } else {
                        panic!("[if_token_test] failed for branch: {}", stringify!($branch));
                    }
                );
                if_token!(
                    $branch($($token)* $($token)* $($token)* $($token)*) => {
                        println!("[if_token_test] passed for branch: multiple {}", stringify!($branch));
                    } else {
                        panic!("[if_token_test] failed for branch: {}", stringify!($branch));
                    }
                );
                
            };
        }
        if_token!{
            () => {
                println!("[if_token_test] passed for branch: ()");
            } else {
                panic!("Matched tokens when there weren't any.")
            }
        }
        if_token!{
            none() => {
                println!("[if_token_test] passed for branch: none");
            } else {
                panic!("Matched tokens when there weren't any.")
            }
        }
        if_token!{
            none(1) => {
                panic!("Didn't match a token when it should have.")
            } else {
                println!("[if_token_test] passed for branch: none(1)");
            }
        }
        if_token_test!(item[struct test {}]);
        if_token_test!(type[bool]);
        if_token_test!(expr[4+3]);
        if_token_test!(block[{
            println!("This won't execute.");
        }]);
        if_token_test!(pat[Option::<String>::Some(name)]);
        if_token_test!(meta[allow(unused)]);
        if_token_test!(ident[TheQuickBrownFoxJumpsOverTheLazyDog]);
        if_token_test!(stmt[block {}]);
        if_token_test!(path[one::two::three::four::five]);
        if_token_test!(lit[true]);
        if_token_test!(lit[12345]);
        if_token_test!(lit["Hello, world"]);
        if_token_test!(lifetime['a]);
    }

    #[test]
    #[allow(unused)]
    #[allow(non_camel_case_types)]
    fn sandbox() {
        enum branches<'a> {
            example_branch { ui: &'a mut usize },
            test_branch,
            DEFAULT,
        }
        use branches::*;
        let mut branch = DEFAULT;
    }
}