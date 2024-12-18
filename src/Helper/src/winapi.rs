use winapi::um::winnls::{
    GetSystemDefaultLCID, GetSystemDefaultLangID, GetSystemDefaultUILanguage,
};

pub static ZH: &str = "2052";
pub static EN: &str = "1033";

pub fn is_zh() -> bool {
    return lang_id() == ZH.to_string();
}

pub fn lc_id() -> String {
    unsafe {
        return GetSystemDefaultLCID().to_string();
    }
}

pub fn lang_id() -> String {
    unsafe {
        return GetSystemDefaultLangID().to_string();
    }
}

pub fn ui_language() -> String {
    unsafe {
        return GetSystemDefaultUILanguage().to_string();
    }
}

pub fn get_welink_lang(cn: &str, en: &str) -> String {
    return format!("'{}'===window.langcode?'{}':'{}'", ZH, cn, en);
}
