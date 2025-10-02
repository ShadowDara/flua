// Color codes for Colorful printing with Ansi Colorcodes
pub const END: &str = "\x1b[0m";

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE:  &str = "\x1b[34m";
pub const PURPLE: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub fn print() {
    // Farben
    println!("\x1b[32mGrün\x1b[0m");      // Grün
    println!("\x1b[31mRot\x1b[0m");       // Rot
    println!("\x1b[33mGelb\x1b[0m");      // Gelb
    println!("\x1b[34mBlau\x1b[0m");      // Blau
    println!("\x1b[36mCyan\x1b[0m");      // Cyan

    // Stile
    println!("\x1b[1mFett\x1b[0m");       // Fett
    println!("\x1b[3mKursiv\x1b[0m");     // Kursiv (nicht überall unterstützt)
    println!("\x1b[4mUnterstrichen\x1b[0m"); // Unterstrichen
}
