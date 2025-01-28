use rt_helper::metadata::get_metadata;

pub const PRODUCT_PAGE: &str = "https://www.huaweicloud.com/product/welink-download.html";

pub fn product_name() -> String {
    get_metadata(env!("CARGO_PKG_NAME"), "winres", "ProductName")
}
pub fn product_version() -> String {
    get_metadata(env!("CARGO_PKG_NAME"), "winres", "ProductVersion")
}
pub fn product_major_version() -> String {
    format!("{}", product_version().split('.').next().unwrap())
}

pub fn assets_list() -> Vec<(&'static str, usize)> {
    // 红蓝共用
    let common_list = vec![
        // 基座页面，包括登录页面、聊天消息页面、通讯录（关注/组织/团队/外部/公众号）
        ("pedestal/index.html", 1),
        // 登录页面-基本设置-设备信息
        ("plugin/basic/deviceInfo/dist/deviceInfo.html", 4),
        // 基座页面-设置弹框
        ("plugin/basic/settingFrame/dist/settingFrame.html", 4),
        // 基座页面-通话与拨号弹框
        ("plugin/callRecord/dist/callRecord.html", 3),
        // 基座页面-通话与拨号弹框-会议详情弹框
        ("plugin/callRecord/dist/meetingDetails.html", 3),
        // 基座页面-更换头像弹框
        ("plugin/contact/dist/avatarSetting.html", 3),
        // 基座页面-通话与拨号弹框
        ("plugin/contact/dist/avatarView.html", 3),
        // 基座页面-联系人详情弹框
        ("plugin/contact/dist/contactDetail.html", 3),
    ];
    let spec_list = match &*product_name() {
        // 仅红We使用
        "WeLink_Desktop" => vec![
            // 聊天消息页面-小微助手侧滑框
            // 小微助手页面
            ("plugin/athenaAssistant/dist/index.html", 3),
        ],
        // 仅蓝We使用
        "WeLink" => vec![
            // Wifi一键连接
            ("plugin/OneWIFI/dist/index.html", 3),
        ],
        _ => vec![],
    };
    common_list.into_iter().chain(spec_list).collect()
}
