use crate::data::{send_msg_replace, toolbar_cb_replace, toolbar_icon_replace, toolbar_replace};
use rt_helper::common::{push_str, replace_str_list};

pub fn main(install_path: String, install_version: String, is_install: bool) {
    // 替换发送消息逻辑
    let send_msg_ori = "t.handSendMsg=async function(e,t,a,n){";
    let send_msg_new = send_msg_replace(send_msg_ori);
    let mut replace_arr = vec![vec![send_msg_ori, &send_msg_new]];
    let mut file_path = format!(
        r"{}\resources\app\pedestal\im\static\js\common.js",
        install_path
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
    file_path = format!(
        r"{}\resources\app\plugin\im\dist\im\static\js\common.js",
        install_path
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
    // 替换菜单逻辑
    let editor_toolbar = "{type:\"WeTask\",toolItemTestid:\"WeTask000123\",title:window.language.WE_TASK_TASK,width:20,height:20,fill:\"#666\",hide:!1,isShow:window.isRedblue,submenuContentInMore:d.default.createElement(g.default,{close:()=>e.open&&e.open(!1)}})}]";
    let editor_toolbar_new = editor_toolbar.replace("]", &toolbar_replace());
    let editor_toolbar_cb = "a=>{switch(a.type){";
    let editor_toolbar_cb_new = toolbar_cb_replace("a", "e");
    replace_arr = vec![
        vec![editor_toolbar, &editor_toolbar_new],
        vec![editor_toolbar_cb, &editor_toolbar_cb_new],
    ];
    file_path = format!(
        r"{}\resources\app\pedestal\im\static\js\3978.js",
        install_path
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
    file_path = format!(
        r"{}\resources\app\plugin\im\dist\im\static\js\3978.js",
        install_path
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
    file_path = format!(
        r"{}\resources\app\pedestal\im\static\css\im.css",
        install_path
    );
    push_str(
        &file_path,
        &toolbar_icon_replace(),
        &install_version,
        is_install,
    );
    file_path = format!(
        r"{}\resources\app\plugin\im\dist\im\static\css\multiwindow.css",
        install_path
    );
    push_str(
        &file_path,
        &toolbar_icon_replace(),
        &install_version,
        is_install,
    )
}
