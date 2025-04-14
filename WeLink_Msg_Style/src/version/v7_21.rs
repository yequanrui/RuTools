use crate::data::{send_msg_replace, toolbar_cb_replace, toolbar_icon_replace, toolbar_replace};
use rt_helper::common::{push_str, replace_str_list};

pub fn main(install_path: String, install_version: String, is_install: bool) {
    // 替换发送消息逻辑
    let send_msg_ori = "t.handSendMsg=async function(e,t,a,n){";
    let send_msg_new = send_msg_replace(send_msg_ori);
    let mut replace_arr = vec![vec![send_msg_ori, &send_msg_new]];
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
    let editor_toolbar = "{type:\"CodeBlockBtn\",toolItemTestid:\"CodeBlockBtn000123\",title:window.language.CODEBLCOK,width:20,height:20,fill:\"#666\",hide:!1,isShow:!0}]";
    let editor_toolbar_new = editor_toolbar.replace("]", &toolbar_replace());
    let editor_toolbar_cb = "l=>{switch(l.type){";
    let editor_toolbar_cb_new = toolbar_cb_replace("l", "e");
    replace_arr = vec![
        vec![editor_toolbar, &editor_toolbar_new],
        vec![editor_toolbar_cb, &editor_toolbar_cb_new],
    ];
    file_path = format!(
        r"{}\resources\app\plugin\im\dist\static\js\3479.js",
        install_path
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
    file_path = format!(
        r"{}\resources\app\plugin\im\dist\static\css\3479.css",
        install_path
    );
    push_str(
        &file_path,
        &toolbar_icon_replace(),
        &install_version,
        is_install,
    );
}
