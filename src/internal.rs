#[inline(always)]
#[should_panic]
#[doc(hidden)]
pub fn _die(code: i32, colour: &str, args: core::fmt::Arguments) -> ! {
    extern crate inline_colorization;
    extern crate errno;
    extern crate tinyvec_string;
    extern crate print_no_std;
    extern crate exit_no_std;

    use inline_colorization::*;
    use core::fmt::Write;
    
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
