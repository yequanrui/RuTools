use rt_helper::i18n::get_data;
use rt_helper::winapi::is_zh;

pub fn get(key: &str) -> &str {
    let res = match key {
        "win_title" => ["WeLink Themes Installation", "WeLink主题安装程序"],
        "community_theme" => ["Install the Official Community Theme. (Only theme switching is supported)", "安装官方社区版主题（仅支持主题切换，不附带功能特性，支持WeLink7.15以上版本，WeLink升级不用重装主题）"],
        "dev_theme" => ["Install the DevTheme. (More new features)", "安装Dev主题（主打新功能特性，支持版本WeLink7.x以上版本，WeLink升级后需要重装主题）"],
        "super_theme" => ["Install the SuperTheme. (More customized themes)", "安装超级主题（主打定制化主题，支持版本WeLink7.x以上版本，WeLink升级后需要重装主题）"],
        "uninstall_theme" => ["Uninstall the theme. (Restore the theme to the original version of WeLink)", "卸载主题（还原回WeLink原版主题）"],
        "download_tips_1" => ["You can download the latest version package of WeLink ", "可在此链接下载WeLink"],
        "download_tips_2" => [" from this link", "最新版本包"],
        "installed_tips_1" => ["The installation program adapts to any version of WeLink ", "安装程序适配WeLink"],
        "installed_tips_2" => [", and theme resources adapt to WeLink ", "任意版本，主题资源适配至WeLink"],
        "installing_tips_1" => ["Installing the theme of ", "正在安装"],
        "installing_tips_2" => ["", "主题"],
        "community" => ["Community Edition", "社区版"],
        "uninstalling_tips" => ["Restoring", "正在还原"],
        "uninstall_tips" => ["Restored back to the original WeLink theme", "已还原回WeLink原版主题"],
        "copying_tips_1" => ["Start copying the resources of ", "开始复制"],
        "copying_tips_2" => ["", "主题资源"],
        "resource_path" => ["Resource Path", "主题资源路径"],
        "target_path" => ["Target Path", "目标路径"],
        "default_config_path" => ["Default Configuration Path", "主题默认配置路径"],
        "user_config_path" => ["User Configuration Path", "用户配置路径"],
        "default_style_path" => ["Default Extended Style Sheet Path", "主题默认扩展样式表路径"],
        "user_style_path" => ["User Extended Style Sheet Path", "用户扩展样式表路径"],
        "config_tips" => ["The configuration file ", "配置文件"],
        "style_tips" => ["Extended style sheet ", "扩展样式表"],
        "skip_tips" => ["already exists. Skip the creation.", "已存在，跳过创建"],
        "non_unicode_tips" => ["User directory path contains non-Unicode characters", "用户目录路径包含非Unicode字符"],
        "no_user_dir_tips" => ["Error getting user directory", "获取用户目录时出错"],
        "no_dir_tips" => ["Error getting current directory", "获取当前目录时出错"],
        _ => get_data(key),
    };
    if is_zh() {
        res[1]
    } else {
        res[0]
    }
}
