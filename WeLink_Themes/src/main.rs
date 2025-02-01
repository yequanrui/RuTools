use console_utils::input::select;
use rt_helper::common::{
    compare_version, copy_res, end_tips, operation_tips, replace_str, wait_for_exit,
};
use rt_helper::console::{info, stdout, warning};
use rt_helper::winapi::is_zh;
use rt_helper::winreg::{find_install_path_and_version, find_user_data_path};
use std::env;
use std::error::Error;
use std::fs::{metadata, remove_file};
use std::path::PathBuf;

mod data;
mod i18n;

#[cfg(test)]
mod tests {
    use rt_helper::metadata::get_metadata;

    #[test]
    fn it_works() {
        println!(
            "{}",
            get_metadata(env!("CARGO_PKG_NAME"), "winres", "ProductName")
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let install_path = preset();

    match data::is_internal_version() {
        true => {
            let options = [
                i18n::get("community_theme"),
                i18n::get("dev_theme"),
                i18n::get("super_theme"),
                i18n::get("uninstall_theme"),
            ];
            let index = select(&operation_tips(), &options).to_owned();
            match Some(index) {
                Some(0) => install_community(install_path),
                Some(1) => install(install_path, "dev", true),
                Some(2) => install(install_path, "superTheme", true),
                Some(3) => revert(install_path, true),
                _ => {}
            }
        }
        false => {
            let options = [i18n::get("dev_theme"), i18n::get("uninstall_theme")];
            let index = select(&operation_tips(), &options).to_owned();
            match Some(index) {
                Some(0) => install(install_path, "dev", false),
                Some(1) => revert(install_path, false),
                _ => {}
            }
        }
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
fn install(install_path: String, theme: &str, is_internal: bool) {
    println!(
        "\n{}{}{}...",
        i18n::get("installing_tips_1"),
        theme,
        i18n::get("installing_tips_2")
    );
    if is_internal {
        uninstall_community(); // 目前自定义主题与社区版不兼容，安装前先卸载社区版
    }
    replace_html(install_path.clone(), false);
    copy_assets(install_path.clone(), theme);
}

/// 还原自定义主题
fn revert(install_path: String, is_internal: bool) {
    println!("\n{}...", i18n::get("uninstalling_tips"));
    replace_html(install_path, true);
    if is_internal {
        uninstall_community();
    }
    println!("\n{}", i18n::get("uninstall_tips"));
}

/// 替换WeLink的静态Html资源
fn replace_html(install_path: String, is_restore: bool) {
    for i in &data::assets_list() {
        let file_path = format!("{}/resources/app/{}", install_path, i);
        println!("{}: {}", i18n::get("start_replace"), info(&file_path));
        let str = format!(
            "<script src=\"{}assets/js/dev.min.js\"></script>",
            "../".repeat(i.matches('/').count())
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
    match env::current_exe() {
        Ok(mut path) => {
            path.pop();
            let mut path1 = path.clone();
            path1.push(theme);
            let mut path0 = PathBuf::from(install_path);
            path0.push("resources");
            path0.push("app");
            println!("- {}: {:?}", i18n::get("resource_path"), info(&path1));
            println!("- {}: {:?}", i18n::get("target_path"), info(&path0));
            if path1.exists() {
                copy_res(&path1, &path0);
            }
            let mut config_path = path0.clone();
            config_path.push("assets");
            let mut extend_path = config_path.clone();
            let mut user_config_path = config_path.clone();
            user_config_path.push(theme);
            let mut user_extend_path = user_config_path.clone();
            user_config_path.set_extension("config.js");
            user_extend_path.set_extension("extend.css");
            extend_path.push("css");
            extend_path.push("extend");
            extend_path.set_extension("css");
            config_path.push("js");
            if is_zh() {
                config_path.push("config");
            } else {
                config_path.push("config-en");
            }
            config_path.set_extension("js");
            if config_path.exists() {
                if !user_config_path.exists() {
                    println!(
                        "\n- {}: {:?}",
                        i18n::get("default_config_path"),
                        info(&config_path)
                    );
                    println!(
                        "- {}: {:?}",
                        i18n::get("user_config_path"),
                        info(&user_config_path)
                    );
                    copy_res(&config_path, &user_config_path);
                } else {
                    println!(
                        "-- {}{}",
                        i18n::get("config_tips"),
                        warning(i18n::get("skip_tips"))
                    );
                }
                if !user_extend_path.exists() {
                    println!(
                        "\n- {}: {:?}",
                        i18n::get("default_style_path"),
                        info(&extend_path)
                    );
                    println!(
                        "- {}: {:?}",
                        i18n::get("user_style_path"),
                        info(&user_extend_path)
                    );
                    copy_res(&extend_path, &user_extend_path);
                } else {
                    println!(
                        "-- {}{}",
                        i18n::get("style_tips"),
                        warning(i18n::get("skip_tips"))
                    );
                }
            }
        }
        Err(e) => eprintln!("{}: {e}", i18n::get("no_dir_tips")),
    }
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
    let software_name = env!("PRODUCT_NAME");
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
        info(data::product_major_version()),
        i18n::get("installed_tips_2"),
        info(env!("PRODUCT_VERSION"))
    );
    // 判断主题资源版本与WeLink版本是否匹配
    if !compare_version(env!("PRODUCT_VERSION"), &install_version) {
        println!(
            "{}{}{}: {}\n",
            i18n::get("download_tips_1"),
            info(data::product_major_version()),
            i18n::get("download_tips_2"),
            info(data::PRODUCT_PAGE)
        )
    }
    install_path
}
