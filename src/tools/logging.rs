// use paste::paste;
//
// #[allow(unused_macros)]
// macro_rules! my_func {
//     // 'm' is for literal matching.
//     // ident can be used here more for alphabets. cant start with a number
//     // but ident can have numbers present else where.
//     ( 'm', $md:literal, $($va:ident),* ) => {{ $(
//             paste! {
//                 [<mod $md _ $va>]();
//             }
//         )*
//     }};
//     ( 't', $nb:literal, "", $va:literal ) => {{
//         paste! {
//             [<th$nb>]::[<thread $va>]();
//         }
//     }};
//     ( 't', $nb:literal, $nest: literal, $va:literal ) => {{
//         paste! {
//             [<th $nb>]::[<th $nest>]::[<thread $va>](); }
//     }};
// }

use std::fmt::{self, Write};

pub struct Logger {
    message: Vec<String>,
}

#[non_exhaustive]
pub enum LogLevel {
    Trace,
    Warn,
    Error,
}

fn process_log_arg(logtype: LogLevel) {

}



