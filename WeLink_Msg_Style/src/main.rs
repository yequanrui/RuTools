mod data;
mod i18n;
mod version;

fn main() {
    println!("Hello, {}!", env!("CARGO_PKG_NAME"));
}

fn preset() {}
