use crate::winapi::is_zh;

pub fn get_data(key: &str) -> [&str; 2] {
    let res;
    match key {
        "welcome_to" => res = ["Welcome to ", "欢迎使用"],
        "notfound_tips_1" => res = ["The installation path of ", "没找到"],
        "notfound_tips_2" => res = [" is not found.", "的安装路径"],
        "notfound_tips_3" => {
            res = [
                "The root directory name of the user data is not found.",
                "没找到用户数据的根目录",
            ]
        }
        "run_command_failed" => res = ["Failed to run command.", "无法运行命令"],
        "open_url_failed" => {
            res = [
                "Failed to launch browser, please make sure default browser is set.",
                "无法启动浏览器，请确保已设置默认浏览器",
            ]
        }
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
    return res;
}

pub fn get(key: &str) -> &str {
    let res = get_data(key);
    return if is_zh() { res[1] } else { res[0] };
}
