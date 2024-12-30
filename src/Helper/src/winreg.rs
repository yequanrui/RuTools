use crate::console::warning;
use crate::i18n::get;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ};
use winreg::RegKey;

/**
 * 通过注册表查询软件的安装版本和路径
 */
pub fn find_install_path_and_version(software_name: &str) -> (String, String) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let software_key = hklm.open_subkey_with_flags("SOFTWARE", KEY_READ).unwrap();
    // 寻找安装路径
    let uninstall_key = software_key
        .open_subkey_with_flags(
            "WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ,
        )
        .unwrap();
    for subkey_name in uninstall_key.enum_keys().map(|k| k.unwrap()) {
        let subkey = uninstall_key
            .open_subkey_with_flags(subkey_name, KEY_READ)
            .unwrap();
        let display_name: String = subkey.get_value("DisplayName").unwrap_or_default();
        let old_software_name = software_name.to_owned().replace("_", " ");
        if display_name.contains(software_name) || display_name.contains(&old_software_name) {
            let install_path = subkey.get_value("InstallLocation");
            let display_version = subkey.get_value("DisplayVersion");
            return (
                install_path.unwrap_or_else(|_| {
                    panic!(
                        "Warning: {}{}{}",
                        get("notfound_tips_1"),
                        warning(software_name),
                        get("notfound_tips_2")
                    )
                }),
                display_version.unwrap_or_default(),
            );
        }
    }
    panic!(
        "Warning: {}{}{}",
        get("notfound_tips_1"),
        warning(software_name),
        get("notfound_tips_2")
    )
}

/**
 * 通过注册表查询软件的用户数据路径
 */
pub fn find_user_data_path(path_str: &str, data_name: &str) -> String {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
        path_str
    );
    let data = hkcu.open_subkey(path).unwrap();
    let value = data.get_value(data_name);
    value.unwrap_or_else(|_| panic!("Warning: {}", get("notfound_tips_3")))
}
