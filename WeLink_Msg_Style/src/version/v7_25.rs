use crate::version::base;

pub fn main(install_path: String, install_version: String, is_install: bool) {
    base::main2(install_path, install_version, is_install, "5070");
}
