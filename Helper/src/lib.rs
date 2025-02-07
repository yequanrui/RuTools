pub mod codec;
pub mod common;
pub mod console;
pub mod i18n;
pub mod metadata;
pub mod reqwest;
pub mod update;
pub mod winapi;
pub mod winreg;

#[cfg(test)]
mod tests {
    use crate::winapi::{lang_id, lc_id, ui_language};
    use crate::winreg::find_install_path_and_version;

    #[test]
    fn it_works() {
        println!("{}/{}/{}", lc_id(), lang_id(), ui_language());
        println!("{:?}", find_install_path_and_version("WeLink"));
    }
}
