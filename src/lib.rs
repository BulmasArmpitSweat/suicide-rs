//! # suicide-rs
//! 
//! This crate allows you to crash a program with fancy output. That's it.
//! 
//! 
#![no_std]
#[doc(hidden)]
pub mod internal;

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
/// extern crate suicide_rs; 
///
/// // Syntax: die!(i32, &str, format_args!(&str, ...));
/// 
/// let Errno: linux_errnos::x86_64::Errno = linux_errnos::x86_64::Errno::EINVAL; // This example uses the linux_errnos crate, but function uses raw i32, so any implementation will work
/// let Colour: &str = inline_colorization::color_red; // This example uses the inline_colorization crate, but function uses &str, so any implementation will work
/// let Msg: &str = "It is good day to be not dead!";
/// suicide_rs::die!(Errno.into_raw(), Colour, Msg);
/// unreachable!("You are dead!");
/// ```
///
/// # Example
/// ```rust
/// extern crate suicide_rs;
/// 
/// let val1: u8 = 10;
/// let val2: u8 = 20;
/// if (val1 + val2) != 35 {
///     suicide_rs::die!(EINVAL, color_red, "It is good day to be not dead!");
/// }
/// unreachable!("You are dead!");
/// ```
#[macro_export]
macro_rules! die {
    ($code:expr, $colour:expr, $($arg:tt)*) => {
        $crate::internal::_die($code, $colour, format_args!($($arg)*))
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
