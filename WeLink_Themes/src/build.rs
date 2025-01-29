#[cfg(windows)]
fn main() {
    extern crate winres;
    let mut res = winres::WindowsResource::new();
    res.set_icon("logo.ico");
    res.compile().unwrap_or_default();
}
