use crate::data::{send_msg_replace, toolbar_cb_replace, toolbar_replace};
use rt_helper::common::replace_str_list;

pub fn main(install_path: String, install_version: String, is_install: bool) {
    // 替换发送消息逻辑
    let send_msg_ori =
        "exports.handSendMsg=async function(content,utMsgInfo,currentRcId,chatAtInfo){";
    let send_msg_new = send_msg_replace(send_msg_ori);
    // 替换菜单逻辑
    let editor_toolbar = "{type:\"WeTask\",toolItemTestid:\"WeTask000123\",title:window.language.WE_TASK_TASK,width:20,height:20,fill:\"#666\",hide:!1,isShow:window.isRedblue&&!(null==session?void 0:session.singled),submenuContentInMore:_react.default.createElement(_SubmenuContent.default,{close:()=>{props.open&&props.open(!1)}})}]";
    let editor_toolbar_new = editor_toolbar.replace("]", &toolbar_replace());
    let editor_toolbar_cb = "data=>{switch(data.type){";
    let editor_toolbar_cb_new = toolbar_cb_replace("data", "props");
    let replace_arr = vec![
        vec![send_msg_ori, &send_msg_new],
        vec![editor_toolbar, &editor_toolbar_new],
        vec![editor_toolbar_cb, &editor_toolbar_cb_new],
    ];
    let file_path = format!(
        r"{}\resources\app\pedestal\im\static\js\im.js",
        install_path
    );
    replace_str_list(
        &file_path,
        replace_arr.clone(),
        &install_version,
        is_install,
    );
}
