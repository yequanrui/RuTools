use crate::winapi::is_zh;

pub fn get_data(key: &str) -> [&str; 2] {
    let res;
    match key {
        "welcome_to" => res = ["Welcome to ", "欢迎使用"],
        "notfound_tips_1" => res = ["The installation path of ", "没找到"],
        "notfound_tips_2" => res = [" is not found.", "的安装路径"],
        "notfound_tips_3" => res = ["The root directory name of the user data is not found.", "没找到用户数据的根目录"],
        _ => res = [key, key],
    }
    return res;
}

pub fn get(key: &str) -> &str {
    let res = get_data(key);
    return if is_zh() { res[1] } else { res[0] };
}
