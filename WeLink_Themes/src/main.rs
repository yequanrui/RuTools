use console_utils::input::select;
use rt_helper::common::{
    compare_version, copy_res, end_tips, operation_tips, replace_str, wait_for_exit,
};
use rt_helper::console::{info, stdout};
use rt_helper::winapi::is_zh;
use rt_helper::winreg::{find_install_path_and_version, find_user_data_path};
use std::env;
use std::error::Error;
use std::fs::{metadata, remove_file};
use std::path::PathBuf;

mod data;
mod i18n;

fn main() -> Result<(), Box<dyn Error>> {
    let install_path = preset();

    let options = [
        i18n::get("community_theme"),
        i18n::get("dev_theme"),
        i18n::get("super_theme"),
        i18n::get("uninstall_theme"),
    ];
    let index = select(&operation_tips(), &options).to_owned();
    match Some(index) {
        Some(0) => install_community(install_path),
        Some(1) => install(install_path, "dev"),
        Some(2) => install(install_path, "superTheme"),
        Some(3) => revert(install_path),
        _ => {}
    }

    end_tips(true);
    wait_for_exit(5);
    Ok(())
}

/// 安装社区版主题
fn install_community(install_path: String) {
    println!(
        "\n{}{}{}...",
        i18n::get("installing_tips_1"),
        info(i18n::get("community")),
        i18n::get("installing_tips_2")
    );
    replace_html(install_path.clone(), true); // 目前自定义主题与社区版不兼容，安装钱先卸载自定义主题
    match env::current_exe() {
        Ok(mut path) => {
            path.pop();
            let mut path1 = path.clone();
            path1.push("community");
            path1.push("themes");
            let mut path0 = PathBuf::from(install_path);
            path0.push("themes");
            println!("\n- {}: {:?}", i18n::get("resource_path"), info(&path1));
            println!("- {}: {:?}", i18n::get("target_path"), info(&path0));
            if path1.exists() {
                copy_res(&path1, &path0);
            }
            match env::var_os("APPDATA") {
                Some(appdata) => {
                    if let Some(appdata_str) = appdata.to_str() {
                        let mut path2 = PathBuf::from(appdata_str);
                        path2.push(find_user_data_path("WeLink_Desktop_Data", "UserDataRoot"));
                        path2.push("themeConfig");
                        path2.set_extension("json");
                        path1.pop();
                        if is_zh() {
                            path1.push("themeConfig-zh");
                        } else {
                            path1.push("themeConfig-en");
                        }
                        path1.set_extension("json");
                        println!("\n- {}: {:?}", i18n::get("resource_path"), info(&path1));
                        println!("- {}: {:?}", i18n::get("target_path"), info(&path2));
                        if path1.exists() {
                            copy_res(&path1, &path2);
                        }
                    } else {
                        eprintln!("{}", i18n::get("non_unicode_tips"));
                    }
                }
                None => eprintln!("{}", i18n::get("no_user_dir_tips")),
            }
        }
        Err(e) => eprintln!("{}: {e}", i18n::get("no_dir_tips")),
    }
}

/// 卸载社区版主题
fn uninstall_community() {
    match env::var_os("APPDATA") {
        Some(appdata) => {
            if let Some(appdata_str) = appdata.to_str() {
                let mut path = PathBuf::from(appdata_str);
                path.push(find_user_data_path("WeLink_Desktop_Data", "UserDataRoot"));
                path.push("themeConfig");
                path.set_extension("json");
                if metadata(&path).is_ok() {
                    remove_file(path).unwrap_or_else(|_| panic!("{}", i18n::get("delete_failed")));
                }
            } else {
                eprintln!("{}", i18n::get("non_unicode_tips"));
            }
        }
        None => eprintln!("{}", i18n::get("no_user_dir_tips")),
    }
}

/// 安装自定义主题
fn install(install_path: String, theme: &str) {
    println!(
        "\n{}{}{}...",
        i18n::get("installing_tips_1"),
        theme,
        i18n::get("installing_tips_2")
    );
    uninstall_community(); // 目前自定义主题与社区版不兼容，安装前先卸载社区版
    replace_html(install_path.clone(), false);
    copy_assets(install_path.clone(), theme);
}

/// 还原自定义主题
fn revert(install_path: String) {
    println!("\n{}...", i18n::get("uninstalling_tips"));
    replace_html(install_path, true);
    uninstall_community();
    println!("\n{}", i18n::get("uninstall_tips"));
}

/// 替换WeLink的静态Html资源
fn replace_html(install_path: String, is_restore: bool) {
    for i in &data::assets_list() {
        let file_path = format!("{}/resources/app/{}", install_path, i.0);
        println!("{}: {}", i18n::get("start_replace"), info(&file_path));
        let str = format!(
            "<script src=\"{}assets/js/dev.min.js\"></script>",
            "../".repeat(i.1)
        );
        replace_str(file_path, &str, "</body>", is_restore);
    }
}

/// 复制主题资源
fn copy_assets(install_path: String, theme: &str) {
    println!(
        "\n{}{}{}...",
        i18n::get("copying_tips_1"),
        info(theme),
        i18n::get("copying_tips_2")
    );
}

/// 预置操作
fn preset() -> String {
    stdout(i18n::get("win_title"));
    println!(
        "{}{} {}\n",
        i18n::get("welcome_to"),
        info(env!("CARGO_PKG_NAME")),
        info(env!("CARGO_PKG_VERSION"))
    );
    let software_name = "WeLink";
    println!(
        "- {}{}{}",
        i18n::get("install_tips_1"),
        info(software_name),
        i18n::get("install_tips_2")
    );
    let (install_path, install_version) = find_install_path_and_version(software_name);
    println!(
        "-- {}: {}",
        i18n::get("installed_version"),
        info(&install_version)
    );
    println!(
        "-- {}: {}\n",
        i18n::get("installed_path"),
        info(&install_path)
    );
    println!(
        "{}{}{}{}\n",
        i18n::get("installed_tips_1"),
        info(data::PROGRAM_VERSION),
        i18n::get("installed_tips_2"),
        info(data::RESOURCE_VERSION)
    );
    // 判断主题资源版本与WeLink版本是否匹配
    if !compare_version(data::RESOURCE_VERSION, &install_version) {
        println!(
            "{}{}{}: {}\n",
            i18n::get("download_tips_1"),
            info(data::PROGRAM_VERSION),
            i18n::get("download_tips_2"),
            info(data::WELINK_7_VERSIONS_PAGE)
        )
    }
    install_path
}
