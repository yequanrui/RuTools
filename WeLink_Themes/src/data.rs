pub const PROGRAM_VERSION: &str = "7.x";
pub const RESOURCE_VERSION: &str = "7.48.6";
pub const WELINK_7_VERSIONS_PAGE: &str = "https://www.huaweicloud.com/product/welink-download.html";
pub fn assets_list() -> Vec<(&'static str, usize)> {
    vec![
        // 基座页面，包括登录页面、聊天消息页面、通讯录（关注/团队/外部/公众号）
        ("pedestal/index.html", 1),
    ]
}
