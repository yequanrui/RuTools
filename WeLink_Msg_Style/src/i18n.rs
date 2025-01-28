use rt_helper::i18n::get_data;
use rt_helper::winapi::is_zh;

pub fn get(key: &str) -> &str {
    let res = match key {
        "win_title" => ["WeLink Msg Style Installation", "WeLink显眼包安装程序"],
        _ => get_data(key),
    };
    if is_zh() {
        res[1]
    } else {
        res[0]
    }
}
