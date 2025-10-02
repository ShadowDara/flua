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
