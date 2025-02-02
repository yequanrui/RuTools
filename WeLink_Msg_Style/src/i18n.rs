use rt_helper::i18n::get_data;
use rt_helper::winapi::{get_welink_lang, is_zh};

pub fn get(key: &str) -> &str {
    let res = match key {
        "win_title" => ["WeLink Msg Style Installation", "WeLink显眼包安装程序"],
        "supported_versions" => ["Compatible with", "当前支持的版本"],
        "install_or_update" => ["Install or Update", "安装/更新"],
        "uninstall" => ["Uninstall", "卸载"],
        "begin_tips_1" => ["Begin", "开始"],
        "begin_tips_2" => [" Msg Style", "显眼包"],
        "no_config_tips_1" => ["No replacement configuration for version ", "没有"],
        "no_config_tips_2" => ["", "版本的替换配置"],
        _ => get_data(key),
    };
    if is_zh() {
        res[1]
    } else {
        res[0]
    }
}

pub fn get_by_lang(key: &str) -> String {
    let res = match key {
        // 菜单显示
        "msg_style" => ["Msg Style", "显眼包"],
        "enabled" => ["Enabled", "已启用"],
        "disabled" => ["Disabled", "未启用"],
        "edit_style" => ["Edit Style", "编辑显眼格式"],
        "more_style" => ["More Style", "更多显眼格式"],
        "reset_style" => ["Reset Style", "重置显眼格式"],
        "added" => ["Added", "已加入"],
        "not_added" => ["Not Added", "未加入"],
        "whitelist" => ["Whitelist", "白名单"],
        "clear_whitelist" => ["Clear Whitelist", "清空白名单"],
        "blacklist" => ["Blacklist", "黑名单"],
        "clear_blacklist" => ["Clear Blacklist", "清空黑名单"],
        "help_link" => ["Help Docs", "帮助文档"],
        // 操作提示
        "example" => ["Example", "示例"],
        "msg_style_enabled" => ["Msg Style Enabled", "显眼包已启用"],
        "msg_style_disabled" => ["Msg Style Disabled", "显眼包已停用"],
        "modify_msg_style" => ["Modify Msg Style", "自定义消息样式"],
        "modify_msg_style_success" => [
            "Modify the custom style successfully.",
            "修改自定义样式成功",
        ],
        "reset_confirm" => [
            "Are you sure you want to reset the custom style?",
            "是否重置自定义样式？",
        ],
        "reset_msg_style_success" => ["Reset the custom style successfully.", "重置自定义样式成功"],
        "whitelist_added" => ["Added to the whitelist.", "已加入白名单"],
        "whitelist_removed" => ["Removed from the whitelist.", "已从白名单移除"],
        "current_whitelist_total" => ["Current total of whitelist", "当前白名单个数"],
        "whitelist_confirm" => ["Clear all whitelist?", "确认清空白名单？"],
        "whitelist_cleared" => ["Cleared whitelist.", "已清空白名单"],
        "blacklist_added" => ["Added to the blacklist.", "已加入黑名单"],
        "blacklist_removed" => ["Removed from the blacklist.", "已从黑名单移除"],
        "current_blacklist" => ["Current total of blacklist", "当前黑名单个数"],
        "blacklist_confirm" => ["Clear all blacklist?", "确认清空黑名单？"],
        "blacklist_cleared" => ["Cleared blacklist.", "已清空黑名单"],
        _ => get_data(key),
    };
    get_welink_lang(res[1], res[0])
}
