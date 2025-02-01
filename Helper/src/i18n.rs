use crate::winapi::is_zh;

pub fn get_data(key: &str) -> [&str; 2] {
    match key {
        "welcome_to" => ["Welcome to ", "欢迎使用"],
        "opera_tips_1" => ["Press ", "请按"],
        "opera_tips_2" => ["↑ ↓", "↑↓键"],
        "opera_tips_3" => [" to select and press ", "选择并按"],
        "opera_tips_4" => ["Enter", "回车键"],
        "opera_tips_5" => [" to continue.", "继续"],
        "install_tips_1" => ["Query the installed version and path of ", "从注册表查询"],
        "install_tips_2" => [" from the registry.", "的安装版本和路径"],
        "installed_version" => ["Version", "安装版本"],
        "installed_path" => ["Path", "安装路径"],
        "update_tips_1" => ["Latest version available", "可用的最新版本"],
        "update_tips_2" => ["Launching the default browser to open the download page. Download the latest version and run it.", "正使用默认浏览器打开下载页面，请下载最新版本后运行"],
        "update_tips_3" => ["It's the latest version.", "已经是最新版本"],
        "notfound_tips_1" => ["The installation path of ", "没找到"],
        "notfound_tips_2" => [" is not found.", "的安装路径"],
        "notfound_tips_3" => ["The root directory name of the user data is not found.", "没找到用户数据的根目录名"],
        "compare_version_tips_1" => ["Compare with the target version, ", "与目标版本"],
        "compare_version_tips_2" => ["it's a perfect match and can be used properly", "完全匹配，可正常使用"],
        "compare_version_tips_3" => ["there is a minor version difference and can ignore and use normally", "有小版本差异，可忽略并正常使用"],
        "compare_version_tips_4" => ["there is a large version difference but the installation does not cause functional problems, you are advised to upgrade to ", "有大版本差异，但安装后不会造成功能性问题，建议升级到版本"],
        "compare_version_tips_5" => ["there is a major version difference, please upgrade to ", "有重大版本差异，请先升级到版本"],
        "start_replace" => ["Start replacing files", "开始替换文件"],
        "read_failed" => ["Failed to read the file", "读取文件失败"],
        "skip_write" => ["The text does not change. Skip writing.", "文本没变化，跳过写入"],
        "write_succeeded" => ["The file is written successfully.", "文件写入成功"],
        "write_failed" => ["Failed to write to file.", "无法写入文件"],
        "create_failed" => ["Failed to create new file.", "创建文件失败"],
        "delete_succeeded" => ["Deleted successfully.", "删除成功"],
        "delete_failed" => ["Deletion failed.", "删除失败"],
        "copy_succeeded" => ["Copying succeeded.", "复制成功"],
        "not_found" => ["Target not found.", "找不到目标"],
        "permission_denied" => ["Not permission to access the resource", "无权访问该资源"],
        "replace_succeeded" => ["Replace succeeded.", "替换成功"],
        "replace_failed" => ["Replace failed.", "替换失败"],
        "no_replace" => ["No replacement required.", "无需替换"],
        "replace_tips_1" => ["The file exists, start to replace.", "文件存在，开始替换"],
        "replace_tips_2" => ["The text exists, remove it.", "文本存在，将其移除"],
        "skip_replace_tips_1" => ["Text to be replaced ", "要替换的文本"],
        "skip_replace_tips_2" => ["The file to be replaced ", "要替换的文件"],
        "skip_replace_tips_3" => ["does not exist, skip replacement.", "不存在，跳过替换"],
        "skip_replace_tips_4" => ["already exists, skip replacement.", "已存在，跳过替换"],
        "backup_tips_1" => ["Start backing up the original file.", "开始备份原文件"],
        "backup_tips_2" => ["The backup file already exists. Restore the file to the original version.", "备份文件已存在，还原回原版"],
        "backup_succeeded" => ["Backup succeeded", "备份成功"],
        "backup_failed" => ["Backup failed", "备份失败"],
        "restore_succeeded" => ["Restore succeeded.", "还原成功"],
        "restore_failed" => ["Restore failed.", "还原失败"],
        "no_backup" => ["No backup required.", "无需备份"],
        "parse_failed" => ["Failed to parse.", "解析失败"],
        "run_command_failed" => ["Failed to run command.", "无法运行命令"],
        "open_url_failed" => ["Failed to launch browser, please make sure default browser is set.", "无法启动浏览器，请确保已设置默认浏览器"],
        "finished_tips" => ["Finished.", "完成！"],
        "restart_tips_1" => ["Restart WeLink", "重启WeLink"],
        "restart_tips_2" => [" for the modification to take effect.", "后生效"],
        "exit_tips_1" => ["The window will close automatically in ", "窗口将在"],
        "exit_tips_2" => [" seconds...", "秒后自动关闭..."],
        "request_failed" => ["Failed to send request.", "请求失败"],
        "response_failed" => ["Failed to read response body.", "无法读取响应正文"],
        "binary_not_exist" => ["The binary package does not exist.", "二进制包不存在"],
        "read_metadata_failed" => ["Unable to read Cargo metadata.", "无法读取Cargo元数据"],
        "package_not_found" => ["The package was not found in the Cargo metadata.", "Cargo元数据中未找到该包"],
        _ => [key, key]
    }
}

pub fn get(key: &str) -> &str {
    let res = get_data(key);
    if is_zh() {
        res[1]
    } else {
        res[0]
    }
}
