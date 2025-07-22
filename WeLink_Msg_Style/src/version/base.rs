use crate::data::{send_msg_replace, toolbar_cb_replace, toolbar_icon_replace, toolbar_replace};
use rt_helper::common::{push_str, replace_str_list};

pub fn main(
    install_path: String,
    install_version: String,
    is_install: bool,
    im_num: &str,
    send_key: &str,
    cb_key: &str,
    open_key: &str,
) {
    // 替换发送消息逻辑
    let send_msg_ori = &format!("t.handSendMsg=async function({send_key}){{");
    let send_msg_new = send_msg_replace(send_msg_ori);
    let mut replace_arr = vec![vec![send_msg_ori, send_msg_new.as_str()]];
    let mut file_path = format!(
        r"{}\resources\app\plugin\im\dist\static\js\common.js",
        install_path
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
    // 替换菜单逻辑
    let editor_toolbar = "{type:\"CodeBlockBtn\",toolItemTestid:\"CodeBlockBtn000123\",title:window.language.CODEBLCOK,width:20,height:20,fill:\"#666\",hide:!1,isShow:!0}";
    let editor_toolbar_new = editor_toolbar.replace("}", &toolbar_replace());
    let editor_toolbar_cb = &format!("{cb_key}=>{{switch({cb_key}.type){{");
    let editor_toolbar_cb_new = toolbar_cb_replace(cb_key, open_key);
    replace_arr = vec![
        vec![editor_toolbar, &editor_toolbar_new],
        vec![editor_toolbar_cb, &editor_toolbar_cb_new],
    ];
    file_path = format!(
        r"{}\resources\app\plugin\im\dist\static\js\{}.js",
        install_path, im_num
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
    file_path = format!(
        r"{}\resources\app\plugin\im\dist\static\css\{}.css",
        install_path, im_num
    );
    push_str(
        &file_path,
        &toolbar_icon_replace(),
        &install_version,
        is_install,
    );
}

pub fn main1(install_path: String, install_version: String, is_install: bool, im_num: &str) {
    main(
        install_path,
        install_version,
        is_install,
        im_num,
        "e,t,a,n",
        "l",
        "e",
    );
}

pub fn main2(install_path: String, install_version: String, is_install: bool, im_num: &str) {
    main(
        install_path,
        install_version,
        is_install,
        im_num,
        "e,t,n,a",
        "n",
        "e",
    );
}
