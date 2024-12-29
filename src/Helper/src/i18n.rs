use crate::winapi::is_zh;

pub fn get_data(key: &str) -> [&str; 2] {
    let res;
    match key {
      "welcome_to" => res = ["Welcome to ", "欢迎使用"],
      "opera_tips_1" => res = ["Press ", "请按"],
      "opera_tips_2" => res = ["↑ ↓", "↑↓键"],
      "opera_tips_3" => res = [" to select and press", "选择并按"],
      "opera_tips_4" => res = ["Enter", "回车键"],
      "opera_tips_5" => res = [" to continue.", "继续"],
      "install_tips_1" => res = ["Query the installed version and path of ", "从注册表查询"],
      "install_tips_2" => res = [" from the registry", "的安装版本和路径"],
      "installed_version" => res = ["Version", "安装版本"],
      "installed_path" => res = ["Path", "安装路径"],
      "update_tips_1" => res = ["Latest version available", "可用的最新版本"],
      "update_tips_2" => res = ["Launching the default browser to open the download page. Download the latest version and run it.", "正使用默认浏览器打开下载页面，请下载最新版本后运行"],
      "update_tips_3" => res = ["It's the latest version.", "已经是最新版本"],
      "notfound_tips_1" => res = ["The installation path of ", "没找到"],
      "notfound_tips_2" => res = [" is not found.", "的安装路径"],
      "notfound_tips_3" => res = ["The root directory name of the user data is not found.", "没找到用户数据的根目录"], 
      "run_command_failed" => res = ["Failed to run command.", "无法运行命令"],
      "open_url_failed" => res = ["Failed to launch browser, please make sure default browser is set、.", "无法启动浏览器，请确保已设置默认浏览器"],
      "compare_version_tips_1"=>res=["Compare with the target version, ","与目标版本"],
      "compare_version_tips_2"=>res=["it's a perfect match and can be used properly","完全匹配，可正常使用"],
      "compare_version_tips_3"=>res=["there is a minor version difference and can ignore and use normally","有小版本差异，可忽略正常使用"],
      "compare_version_tips_4"=>res=["there is a large version difference but the installation does not cause functional problems, you are advised to upgrade to ","有大版本差异，但安装后不会造成功能性问题，建议升级到版本"],
      "compare_version_tips_5"=>res=["there is a major version difference, please upgrade to ","有重大版本差异，请先升级到版本"],
      "finished_tips" => res = ["Finished.", "完成！"],
      "restart_tips_1" => res = ["Restart WeLink", "重启WeLink"],
      "restart_tips_2" => res = [" for the modification to take effect.", "后生效"],
      "exit_tips_1" => res = ["The window will close automatically in ", "窗口将在"],
      "exit_tips_2" => res = [" seconds...", "秒后自动关闭..."],
      "request_failed" => res = ["Failed to send request.", "请求失败"],
      "response_failed" => res = ["Failed to read response body.", "无法读取响应正文"],
      "binary_not_exist" => res = ["The binary package does not exist.", "二进制包不存在"],
      _ => res = [key, key],
    }
    res
}

pub fn get(key: &str) -> &str {
    let res = get_data(key);
    if is_zh() {
        res[1]
    } else {
        res[0]
    }
}
