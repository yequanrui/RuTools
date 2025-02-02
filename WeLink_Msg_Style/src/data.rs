use crate::i18n::get_by_lang;

/// 显眼包格式分享表格链接
pub const MSG_STYLE_TABLE: &str = "https://kdocs.cn/l/cs7U6nzAF1GG";

/// 显眼包帮助文档链接
pub const MSG_STYLE_HELP: &str = "";

/// 显眼包菜单键值（供type和testid字段使用）
pub const TYPE_ENABLE: &str = "XianYanBaoEnable";
pub const TYPE_MODIFY: &str = "XianYanBaoModify";
pub const TYPE_MORE: &str = "XianYanBaoMore";
pub const TYPE_RESET: &str = "XianYanBaoReset";
pub const TYPE_ENABLE_WHITE: &str = "XianYanBaoEnableWhite";
pub const TYPE_CLEAR_WHITE: &str = "XianYanBaoClearWhite";
pub const TYPE_ENABLE_BLACK: &str = "XianYanBaoEnableBlack";
pub const TYPE_CLEAR_BLACK: &str = "XianYanBaoClearBlack";
pub const TYPE_HELP: &str = "XianYanBaoHelp";
pub const TEST_ID_SUFFIX: &str = "000123";

/// 获取当前会话ID的代码
pub const GET_CURRENT_ID: &str =
    "(window.store.getState().im||window.store.getState()).session.current.rcId";

/// 发送消息的代码
pub fn send_msg_replace(replace_str: &str) -> String {
    let mut res = String::from(replace_str);
    res.push_str("const wla=JSON.parse(localStorage.getItem('welink_msg_config://whitelist')||'[]'),bla=JSON.parse(localStorage.getItem('welink_msg_config://blacklist')||'[]');let sc=localStorage.getItem('welink_msg_config://style_config');if(!localStorage.getItem('welink_msg_config://feature_enable')||(wla.length&&wla.includes(a))||!wla.includes(a)&&bla.includes(a)))sc='';''===sc.trim()||''===e.trim()||e.includes('<')&&/<img [^>]*role=\"(picture|file)\"[^>]*>/gi.test(e)||(e=sc.replaceAll({{ORI_CONTENT}}',e));");
    res
}

/// 渲染工具栏菜单的代码
pub fn toolbar_replace() -> String {
    let mut res = String::new();
    let pub_str = ",width:20,height:20,fill:\"#666\",hide:!1,isShow:!0}";
    res.push_str(&format!(
        ",{{type:\"{TYPE_ENABLE}\",toolItemTestid:\"{TYPE_ENABLE}{TEST_ID_SUFFIX}\",title:({})+\": \"+(localStorage.getItem('welink_msg_config://feature_enable')?{}:{})",
        get_by_lang("msg_style_enable"),
        get_by_lang("enabled"),
        get_by_lang("disabled"),
    ));
    res.push_str(pub_str);
    let new_pub_str = pub_str.replace(
        "isShow:!0",
        "isShow:localStorage.getItem('welink_msg_config://feature_enable')",
    );
    res.push_str(&format!(
        ",{{type:\"{TYPE_MODIFY}\",toolItemTestid:\"{TYPE_MODIFY}{TEST_ID_SUFFIX}\",title:{}",
        get_by_lang("edit_style"),
    ));
    res.push_str(&new_pub_str);
    res.push_str(&format!(
        ",{{type:\"{TYPE_RESET}\",toolItemTestid:\"{TYPE_RESET}{TEST_ID_SUFFIX}\",title:{}",
        get_by_lang("reset_style"),
    ));
    res.push_str(&new_pub_str);
    res.push_str(&format!(
        ",{{type:\"{TYPE_ENABLE_WHITE}\",toolItemTestid:\"{TYPE_ENABLE_WHITE}{TEST_ID_SUFFIX}\",title:({})+\": \"+(localStorage.getItem('welink_msg_config://whitelist')?.includes({GET_CURRENT_ID})?{}:{})",
        get_by_lang("whitelist"),
        get_by_lang("added"),
        get_by_lang("not_added"),
    ));
    res.push_str(&new_pub_str);
    res.push_str(&format!(
        ",{{type:\"{TYPE_CLEAR_WHITE}\",toolItemTestid:\"{TYPE_CLEAR_WHITE}{TEST_ID_SUFFIX}\",title:{}",
        get_by_lang("clear_whitelist"),
    ));
    res.push_str(&new_pub_str.replace(
        "}",
        "&&localStorage.getItem('welink_msg_config://whitelist')}",
    ));
    res.push_str(&format!(
        ",{{type:\"{TYPE_ENABLE_BLACK}\",toolItemTestid:\"{TYPE_ENABLE_BLACK}{TEST_ID_SUFFIX}\",title:({})+\": \"+(localStorage.getItem('welink_msg_config://blacklist')?.includes({GET_CURRENT_ID})?{}:{})",
        get_by_lang("blacklist"),
        get_by_lang("added"),
        get_by_lang("not_added"),
    ));
    res.push_str(&new_pub_str.replace(
        "}",
        "&&localStorage.getItem('welink_msg_config://whitelist')}",
    ));
    res.push_str(&format!(
        ",{{type:\"{TYPE_CLEAR_BLACK}\",toolItemTestid:\"{TYPE_CLEAR_BLACK}{TEST_ID_SUFFIX}\",title:{}",
        get_by_lang("clear_blacklist"),
    ));
    res.push_str(&new_pub_str.replace(
        "}",
        "&&!localStorage.getItem('welink_msg_config://whitelist')&&localStorage.getItem('welink_msg_config://blacklist')}",
    ));
    res.push_str(&format!(
        ",{{type:\"{TYPE_HELP}\",toolItemTestid:\"{TYPE_HELP}{TEST_ID_SUFFIX}\",title:{}",
        get_by_lang("help_link"),
    ));
    res.push_str(&new_pub_str);
    res.push_str("]");
    res
}

/// 工具栏菜单点击事件的代码
///
/// key1取替换参数的首单词
/// key2取".open"前的单词
pub fn toolbar_cb_replace(key1: &str, key2: &str) -> String {
    let mut res = String::new();
    let origin_msg_config = "'<span style=\"display:block;color:transparent !important;background:linear-gradient(135deg,#3C51A6,#207AB3,#169E6C,#6CA83B,#B58200,B54E04,#B02121.#AD215B,#572DB3);-webkit-background-size:100% 100%;-webkit-background-clip:text;-webkit-text-stroke:.1px transparent;letter-spacing:2px;\">{{ORI_CONTENT}}</span>'";
    res.push_str(&format!("async ({})=>{{", key1));
    res.push_str("const pc=e=>window.Pedestal.callMethod('method://pedestal/confirm',{dialogId:1223,content:e}),pt=e=>window.Pedestal.callMethod('method://pedestal/toast',{content:e}),pnp=e=>window.Pedestal.callMethod('method://pedestal/notifyPrompt',e),fe='welink_msg_config://feature_enable',mc='welink_msg_config://style_config',wl='welink_msg_config://whitelist',bl='welink_msg_config://blacklist';");
    res.push_str(&format!("switch({}.type){{", key1));
    // 启用/禁用显眼包
    res.push_str(&format!(
        "case'{TYPE_ENABLE}':if(localStorage.getItem(fe)){{localStorage.removeItem(fe),pt({})}}else{{localStorage.setItem(fe,'true');pt({}),!localStorage.getItem(mc)&&localStorage.setItem(mc,{origin_msg_config})}}e.open&&e.open(!1);break;",
        get_by_lang("msg_style_disabled"),
        get_by_lang("msg_style_enabled"),
    ));
    // 编辑显眼格式
    res.push_str(&format!(
        "case'{TYPE_MODIFY}':{{const o=await pnp({{dialogId:1223,title:{},inputParam:{{inputType:'text',placeholders:'{}: <span>{{{{ORI_CONTENT}}}}</span>',maxLength:1e5,defaultValue:(localStorage.getItem(mc)||'')}}}});if(o.ret){{const e=o.param.input_type_text.trim();localStorage.setItem(mc,e);pt({})}}e.open&&e.open(!1)}}break;",
        get_by_lang("modify_msg_style"),
        get_by_lang("example"),
        get_by_lang("modify_msg_style_success"),
    ));
    // 更多显眼格式
    res.push_str(&format!("case'{TYPE_MORE}':window.Pedestal.callMethod('method://pedestal/openUrl','{MSG_STYLE_TABLE}'),e.open&&e.open(!1);break;"));
    // 重置显眼格式
    res.push_str(&format!(
        "case'{TYPE_RESET}':{{const ac=await pc({});if(ac.ret){{localStorage.setItem(mc,{origin_msg_config});pt({})}}e.open&&e.open(!1)}}break;",
        get_by_lang("reset_confirm"),
        get_by_lang("reset_msg_style_success"),
    ));
    // 启用/禁用白名单
    res.push_str(&format!(
        "case'{TYPE_ENABLE_WHITE}':{{const wla=JSON.parse(localStorage.getItem(wl)||'[]'),n={GET_CURRENT_ID};let s='';if(wla.includes(n)){{wla.splice(wla.indexOf(n),1);s=({})}}else{{wla.push(n);s=({})}}localStorage.setItem(wl,JSON.stringify(wla));!wla.length&&localStorage.removeItem(wl);pt(`${{s}}, ${{{}}}: ${{wla.length}}`);e.open&&e.open(!1)}}break;",
        get_by_lang("whitelist_removed"),
        get_by_lang("whitelist_added"),
        get_by_lang("current_whitelist_total"),
    ));
    // 清空白名单
    res.push_str(&format!(
        "case'{TYPE_CLEAR_WHITE}':{{const ac=await pc({});if(ac.ret){{localStorage.removeItem(wl);pt({})}}e.open&&e.open(!1)}}break;",
        get_by_lang("whitelist_confirm"),
        get_by_lang("whitelist_cleared"),
    ));
    // 启用/禁用黑名单
    res.push_str(&format!(
        "case'{TYPE_ENABLE_BLACK}':{{const bla=JSON.parse(localStorage.getItem(bl)||'[]'),n={GET_CURRENT_ID};let s='';if(bla.includes(n)){{bla.splice(bla.indexOf(n),1);s=({})}}else{{bla.push(n);s=({})}}localStorage.setItem(bl,JSON.stringify(bla));!bla.length&&localStorage.removeItem(bl);pt(`${{s}}, ${{{}}}: ${{bla.length}}`);e.open&&e.open(!1)}}break;",
        get_by_lang("blacklist_removed"),
        get_by_lang("blacklist_added"),
        get_by_lang("current_blacklist_total"),
    ));
    // 清空黑名单
    res.push_str(&format!(
        "case'{TYPE_CLEAR_BLACK}':{{const ac=await pc({});if(ac.ret){{localStorage.removeItem(bl);pt({})}}e.open&&e.open(!1)}}break;",
        get_by_lang("blacklist_confirm"),
        get_by_lang("blacklist_cleared"),
    ));
    // 帮助文档
    res.push_str(&format!("case'{TYPE_HELP}':window.Pedestal.callMethod('method://pedestal/openUrl','{MSG_STYLE_HELP}'),e.open&&e.open(!1);break;"));
    res.replace("e.open", &format!("{key2}.open"))
}

/// 工具栏菜单图标的样式
pub fn toolbar_icon_class(type_key: &str, icon: &str) -> String {
    let filter: &str = match type_key {
        TYPE_ENABLE_BLACK | TYPE_CLEAR_BLACK => "filter:grayscale(1);",
        _ => "",
    };
    format!("[class*='toobarMore_iconWrap__'][data-testid='{type_key}{TEST_ID_SUFFIX}']:before{{content:\"{icon}\";{filter}}}")
}

/// 工具栏菜单图标集的样式
pub fn toolbar_icon_replace() -> String {
    let mut res = String::from("[class*='toobarMore_btnText__']{white-space:nowrap;}");
    res.push_str(&toolbar_icon_class(TYPE_ENABLE, "🎨"));
    res.push_str(&toolbar_icon_class(TYPE_MODIFY, "📝"));
    res.push_str(&toolbar_icon_class(TYPE_MORE, "🔗"));
    res.push_str(&toolbar_icon_class(TYPE_RESET, "♻️"));
    res.push_str(&toolbar_icon_class(TYPE_ENABLE_WHITE, "🛡️"));
    res.push_str(&toolbar_icon_class(TYPE_CLEAR_WHITE, "🈳"));
    res.push_str(&toolbar_icon_class(TYPE_ENABLE_BLACK, "🛡️"));
    res.push_str(&toolbar_icon_class(TYPE_CLEAR_BLACK, "🈳"));
    res.push_str(&toolbar_icon_class(TYPE_HELP, "📄"));
    res
}
