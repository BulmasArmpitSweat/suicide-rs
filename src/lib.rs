use inline_colorization::*;
use std::fmt::Write;

#[inline]
#[should_panic]
fn _die(code: linux_errnos::x86_64::Errno, colour: &str, args: std::fmt::Arguments) -> ! {
    let errno_code: i32 = code.into_raw();
    errno::set_errno(errno::Errno(errno_code));
    let mut output = String::new();
    output.write_fmt(args).unwrap();
    panic!(
        "{style_bold}{}[ ERROR ({}:{}) ]: {}: {}, {}{style_reset}{color_reset}",
        colour,
        file!(),
        line!(),
        errno_code,
        code.name().unwrap_or("(none)"),
        output
    );
}

/// Abort program with fancy output
///
/// Like panic, but automatically sets the errno to the code and exits with extra information for your bittersweet pleasure
/// You can thank me later.
///
/// # Usage
/// ```rust
/// let Errno: linux_errnos::x86_64::Errno = linux_errnos::x86_64::Errno::EINVAL; // Uses linux_errnos crate for errnos
/// let Colour: &str = inline_colorization::color_red; // Uses inline_colorization crate for colours
/// let Msg: &str = "It is good day to be not dead!";
/// die!(Errno, Colour, Msg);
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
    ($code:ident, $colour:ident, $($arg:tt)*) => {
        $crate::_die(linux_errnos::x86_64::Errno::$code, inline_colorization::$colour, format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_works() {
        let val1: u8 = 10;
        let val2: u8 = 20;
        if (val1 + val2) != 35 {
            die!(EINVAL, color_red, "It is good day to be not dead!");
        }
        unreachable!("You are dead!");
    }
}
