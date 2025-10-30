// fluaoslib Crate
// a library Create for flua which contains some OS depending Code
// and other Stuff

pub mod folderdialog;

// Module for enabling UTF8 for windows
#[cfg(windows)]
pub mod windows_utf8;
