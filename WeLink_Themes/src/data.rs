// 产品包下载链接
pub fn product_download_url() -> &'static str {
    if is_internal_version() {
        "https://onebox.huawei.com/v/14893661f51ee0f762b9d1dffe8b5aa0/list#linkFolder/1/29"
    } else {
        "https://www.huaweicloud.com/product/welink-download.html"
    }
}

/// 获取主版本号
pub fn product_major_version() -> String {
    format!("{}.x", env!("PRODUCT_VERSION").split('.').next().unwrap())
}

/// 是否内部版本
pub fn is_internal_version() -> bool {
    env!("PRODUCT_NAME") == "WeLink_Desktop"
}

/// html资源列表
pub fn assets_list() -> Vec<&'static str> {
    // 红蓝共用
    let common_list = vec![
        // 基座页面，包括登录页面、聊天消息页面、通讯录（关注/组织/团队/外部/公众号）
        "pedestal/index.html",
        // 登录页面-基本设置-设备信息
        "plugin/basic/deviceInfo/dist/deviceInfo.html",
        // 基座页面-设置弹窗
        "plugin/basic/settingFrame/dist/settingFrame.html",
        // 基座页面-通话与拨号弹框
        "plugin/callRecord/dist/callRecord.html",
        // 基座页面-通话与拨号弹框-会议详情弹框
        "plugin/callRecord/dist/meetingDetails.html",
        // 基座页面-更换头像弹框
        "plugin/contact/dist/avatarSetting.html",
        // 基座页面-查看头像弹框
        "plugin/contact/dist/avatarView.html",
        // 基座页面-联系人详情弹框
        "plugin/contact/dist/contactDetail.html",
        // 基座页面-选择联系人弹框
        // 基座页面-添加关注弹框
        // 基座页面-添加成员弹框
        // 基座页面-创建群聊弹框
        // 基座页面-创建团队弹框
        // 基座页面-选择会议成员弹框
        // 基座页面-其他场景中添加成员弹框
        // 基座页面-通话中添加成员弹框
        "plugin/ContactChooser/dist/contact-chooser.html",
        // 基座页面-顶部全局搜索框
        "plugin/globalSearch/dist/global-search.html",
        // 基座页面-设置弹窗-清理群组弹框
        "plugin/im/dist/batchLeaveGroup.html",
        // 聊天消息页面-群聊的聊天记录
        "plugin/im/dist/combineMessage.html",
        // 基座页面-设置弹窗-更换背景弹框
        // 聊天消息页面-更换背景弹框
        "plugin/im/dist/customBackground.html",
        // 聊天消息页面-转发至弹框
        "plugin/im/dist/forwardModle.html",
        // 聊天消息页面-讨论组/群组-设置弹框
        "plugin/im/dist/groupInfo.html",
        // 聊天消息页面-聊天记录弹框
        "plugin/im/dist/historyManager.html",
        // 聊天消息页面-群成员导入弹框
        "plugin/im/dist/importMembers.html",
        // 桌面右下角消息提醒
        "plugin/im/dist/messageBox.html",
        // 聊天消息页面-独立聊天弹窗
        "plugin/im/dist/multiwindow.html",
        // 聊天消息页面-群二维码弹框
        "plugin/im/dist/qrCodeDialog.html",
        // 聊天消息页面-个人聊天设置弹框
        "plugin/im/dist/singleChatSetting.html",
        // 云空间页面
        // 聊天消息页面-团队空间弹框
        "plugin/onebox/web/dist/index.html",
        // 图片查看器
        "plugin/previewPicture/dist/index.html",
    ];
    let spec_list = if is_internal_version() {
        // 仅红We（内部版本）使用
        vec![
            // 聊天消息页面-小微助手侧滑框
            // 小微助手页面
            "plugin/athenaAssistant/dist/index.html",
            // 群成员选择框
            "plugin/im/dist/memberChooser.html",
            // 桌面右下角语音通话/多媒体会议提醒
            "plugin/UCconference/dist/videoConfView.html",
        ]
    } else {
        // 仅蓝We使用
        vec![
            // 登录页面-基本设置-网络检测弹窗
            "plugin/basic/networkdetection/dist/networkdetection.html",
            // 业务页面
            "plugin/business/dist/index.html",
            // 日历页面
            "plugin/calendar/web/index.html",
            // 会议页面
            "plugin/conference/dist/index.html",
            // 预约会议弹窗
            "plugin/conference/dist/bookMeeting.html",
            // 加入会议弹窗
            "plugin/conference/dist/joinMeeting.html",
            // 投影弹窗
            "plugin/eshare/dist/eshare.html",
            // 直播弹窗
            "plugin/live/dist/index.html",
            // Wifi一键连接弹窗
            "plugin/OneWIFI/dist/index.html",
            // 问题反馈弹窗
            "plugin/feedback/dist/feedback.html",
        ]
    };
    common_list
        .iter()
        .chain(spec_list.iter())
        .cloned()
        .collect()
}
