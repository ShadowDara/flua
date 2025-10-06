// Build Information

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/heart.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // Nichts tun auf anderen Plattformen
}
