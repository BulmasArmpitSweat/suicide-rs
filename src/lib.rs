//! # suicide-rs
//! 
//! This crate allows you to crash a program with fancy output. That's it.
//! 
//! 
#![no_std]

extern crate inline_colorization;
extern crate errno;
extern crate tinyvec_string;
extern crate print_no_std;
extern crate exit_no_std;

use inline_colorization::*;
use core::fmt::Write;

#[inline(always)]
#[should_panic]
fn _die(code: i32, colour: &str, args: core::fmt::Arguments) -> ! {
    errno::set_errno(errno::Errno(code));
    let mut output: tinyvec_string::tinystring::TinyString<[u8; 32]> = tinyvec_string::tinystring::TinyString::new();
    output.write_fmt(args).unwrap();
    // A1-Triard, my saviour: https://crates.io/users/A1-Triard
    // Thanks you for the so useful print_no_std and exit_no_std
    // I'm gonna suck your di-
    print_no_std::println!(
        "{style_bold}{}[ ERROR ({}:{}) ]: {}: {}, {}{style_reset}{color_reset}",
        colour,
        file!(),
        line!(),
        code,
        errno::errno(),
        output
    );
    exit_no_std::exit(u8::MAX);
}

/// Abort program with fancy output
///
/// Like panic, but automatically sets the errno to the code and exits with extra information for your bittersweet pleasure
/// You can thank me later.
///
/// Internally, this macro is a wrapper for a function that uses
/// `tinyvec_string::tinystring::TinyString<A>` internally, allowing for a passed
/// string to be heap-allocated up to 32 characters long, before falling back to the heap.
/// 
/// # Usage
/// ```rust
/// // Syntax: die!(i32, &str, format_args!(&str, ...));
/// 
/// let Errno: linux_errnos::x86_64::Errno = linux_errnos::x86_64::Errno::EINVAL; // This example uses the linux_errnos crate, but function uses raw i32, so any implementation will work
/// let Colour: &str = inline_colorization::color_red; // This example uses the inline_colorization crate, but function uses &str, so any implementation will work
/// let Msg: &str = "It is good day to be not dead!";
/// die!(Errno.into_raw(), Colour, Msg);
/// unreachable!("You are dead!");
/// ```
///
/// # Example
/// ```rust
/// let val1: u8 = 10;
/// let val2: u8 = 20;
/// if (val1 + val2) != 35 {
///     die!(EINVAL, color_red, "It is good day to be not dead!");
/// }
/// unreachable!("You are dead!");
/// ```
#[macro_export]
macro_rules! die {
    ($code:expr, $colour:expr, $($arg:tt)*) => {
        $crate::_die($code, $colour, format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use linux_errnos::x86_64::Errno;

    #[test]
    #[should_panic]
    fn it_works() {
        let val1: u8 = 10;
        let val2: u8 = 20;
        if (val1 + val2) != 35 {
            die!(Errno::EINVAL.into_raw(), inline_colorization::color_red, "It is good day to be not dead!");
        }
        unreachable!("You are dead!");
    }
}
