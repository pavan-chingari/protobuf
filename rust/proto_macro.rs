// #![feature(trace_macros)]
// #![feature(log_syntax)]

// trace_macros!(true);
#[macro_export]
macro_rules! proto {
    // nested message,
    (@msg $msg:ident $submsg:ident : { $field:ident : $($value:tt)* }, $($rest:tt)*) => {
        proto!(@msg $msg $submsg : { $field : $($value)* });
        proto!(@msg $msg $($rest)*);
    };

    // nested message
    (@msg $msg:ident $submsg:ident : { $field:ident : $($value:tt)* }) => {
        {
            let mut $msg = $crate::__macros::paste!{$msg.[<$submsg _mut>]()};
            proto!(@msg $msg $field : $($value)*);
        }
    };

    // empty nested message,
    (@msg $msg:ident $submsg:ident : { }, $($rest:tt)*) => {
        proto!(@msg $msg $submsg : { });
        proto!(@msg $msg $($rest)*);
    };
    // empty nested message
    (@msg $msg:ident $submsg:ident : { }) => {
        {
            let mut $msg = $crate::__macros::paste!{$msg.[<$submsg _mut>]()};
        }
    };

    // field: expr,
    (@msg $msg:ident $ident:ident : $expr:expr, $($rest:tt)*) => {
        // delegate without ,
        proto!(@msg $msg $ident : $expr);
        proto!(@msg $msg $($rest)*);
    };

    // field: expr
    (@msg $msg:ident $ident:ident : $expr:expr) => {
        $crate::__macros::paste!{
            $msg.[<set_ $ident>]($expr);
        }
    };

    () => {};
    (@msg $msg:ident) => {};

    // entry point
    ($ident:ident { $($tt:tt)* }) => {
        {
            let mut message = $ident::new();
            proto!(@msg message $($tt)*);
            message
        }
    };
}
