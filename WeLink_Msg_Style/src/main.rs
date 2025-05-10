use clap::{Args, Parser};
use console_utils::input::select;
use rt_helper::common::{
    end_tips, is_internal_version, judge_version, open_url, operation_tips, wait_for_exit,
};
use rt_helper::console::{info, stdout, warning};
use rt_helper::reqwest::{get_latest_package_ids, OPENX_DOWNLOAD_PAGE, OPENX_PROJECT_ID};
use rt_helper::winreg::find_install_path_and_version;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;

mod data;
mod i18n;
mod version;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// 执行选项
    #[command(flatten)]
    options: Options,

    /// 是否跳过版本检测
    #[arg(short, long)]
    skip: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct Options {
    /// 安装/更新
    #[arg(short, long)]
    install: bool,

    /// 卸载
    #[arg(short, long)]
    uninstall: bool,
}

#[cfg(test)]
mod tests {
    use crate::data::{send_msg_replace, toolbar_cb_replace, toolbar_replace};

    #[test]
    fn it_works() {
        let send_msg_ori = "t.handSendMsg=async function(e,t,a,n){";
        let editor_toolbar = "{type:\"CodeBlockBtn\",toolItemTestid:\"CodeBlockBtn000123\",title:window.language.CODEBLCOK,width:20,height:20,fill:\"#666\",hide:!1,isShow:!0}]";
        println!(
            "{}\n{}\n{}",
            send_msg_replace(send_msg_ori),
            editor_toolbar.replace("]", &toolbar_replace()),
            toolbar_cb_replace("l", "e")
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let (install_path, install_version) = preset(args.skip);
    // 使用BTreeMap可以保持键的插入顺序
    let mut compatible_versions: BTreeMap<&str, fn(String, String, bool)> = BTreeMap::new();
    if is_internal_version(env!("PRODUCT_NAME")) {
        // 仅红We（内部版本）使用
        compatible_versions.insert("7.11.3", version::v7_11::main);
        compatible_versions.insert("7.12.7", version::v7_12::main);
        compatible_versions.insert("7.13.12", version::v7_13::main);
        compatible_versions.insert("7.14.8", version::v7_14::main);
        compatible_versions.insert("7.15.8", version::v7_15_16_48::main);
        compatible_versions.insert("7.16.8", version::v7_15_16_48::main);
        compatible_versions.insert("7.17.16", version::v7_17_49::main);
        compatible_versions.insert("7.18.7", version::v7_18_19::main);
        compatible_versions.insert("7.19.6", version::v7_18_19::main);
        compatible_versions.insert("7.20.6", version::v7_20_50_51::main);
        compatible_versions.insert("7.21.6", version::v7_21::main);
    } else {
        // 仅蓝We使用
        compatible_versions.insert("7.48.6", version::v7_15_16_48::main);
        compatible_versions.insert("7.49.6", version::v7_17_49::main);
        compatible_versions.insert("7.50.3", version::v7_20_50_51::main);
        compatible_versions.insert("7.51.6", version::v7_20_50_51::main);
    }
    let keys: Vec<&str> = compatible_versions.clone().into_keys().collect();
    println!(
        "{}: {}\n",
        i18n::get("supported_versions"),
        info(keys.join(", "))
    );
    // 判断版本是否匹配（忽略小版本差异）并开始执行
    let mut is_success = false;
    let mut version = install_version.as_str();
    for &key in &keys {
        let v = version.rsplit_once(".").unwrap();
        if key != version && key.starts_with(&format!("{}.", v.0)) {
            version = key;
            println!(
                "{}{}\n",
                i18n::get("compare_version_tips_1"),
                warning(i18n::get("compare_version_tips_3"))
            );
            break;
        }
    }
    match compatible_versions.get(version) {
        Some(&value) => {
            let options = &args.options;
            let is_install = match (options.install, options.uninstall) {
                (true, _) => true,
                (_, true) => false,
                _ => {
                    let options = [i18n::get("install_or_update"), i18n::get("uninstall")];
                    let index = select(&operation_tips(), &options).to_owned();
                    println!(
                        "\n{}{}{}...\n",
                        i18n::get("begin_tips_1"),
                        info(options[index]),
                        i18n::get("begin_tips_2")
                    );
                    index == 0
                }
            };
            value(install_path.clone(), install_version.clone(), is_install);
            is_success = true;
        }
        None => println!(
            "{}{}{}\n",
            i18n::get("no_config_tips_1"),
            info(&install_version),
            i18n::get("no_config_tips_2")
        ),
    }
    // 执行结束
    end_tips(is_success);
    wait_for_exit(5);
    Ok(())
}

/// 预置操作
fn preset(skip_check: bool) -> (String, String) {
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
    // 检查安装程序是否有最新版本
    if !skip_check && is_internal_version(env!("PRODUCT_NAME")) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let package = get_latest_package_ids(OPENX_PROJECT_ID, env!("CARGO_PKG_NAME"))
                .await
                .unwrap();
            if judge_version(env!("CARGO_PKG_VERSION"), package[1].as_str()) {
                println!(
                    "{}: {}\n{}\n",
                    i18n::get("update_tips_1"),
                    info(package[1].clone()),
                    i18n::get("update_tips_2")
                );
                open_url(OPENX_DOWNLOAD_PAGE);
                wait_for_exit(5);
            } else {
                println!("{}\n", i18n::get("update_tips_3"));
            }
        });
    }
    (install_path, install_version)
}
