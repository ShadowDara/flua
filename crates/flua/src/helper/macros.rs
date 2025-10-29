// Some Macros for Flua

// Deprecated Warning for Outdated Stuff in Color Yellow
#[macro_export]
macro_rules! deprecated {
    ($name:expr, $version:expr, $msg:expr) => {
        println!(
            "\x1b[33m[DEPRECATED-WARNING] '{}' is deprecated since version: {}\n[DEPRECATED-WARNING] {}\x1b[0m",
            $name, $version, $msg
        );
    };

    ($name:expr, $version:expr) => {
        println!(
            "\x1b[33m[DEPRECATED-WARNING] '{}' is deprecated since version: {}\x1b[0m",
            $name, $version
        );
    };
}

// Macro for Error which should made an Issue for or uninplented Stuff
#[macro_export]
macro_rules! doissue {
    ($msg:expr, $code:expr) => {
        println!(
            "This Feature is unimplemented!\n{}\nError Code: {}\nPlease open an Issue on GitHub with the error code if you see this!\nhttps://github.com/ShadowDara/flua",
            $msg, $code
        );
    };

    ($msg:expr) => {
        println!(
            "This Feature is unimplemented!\n{}\nPlease open an Issue on GitHub with the error code if you see this!\nhttps://github.com/ShadowDara/flua",
            $msg
        );
    };
}
