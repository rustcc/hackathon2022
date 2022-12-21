use once_cell::sync::Lazy;
use regex::Regex;

pub static EMAIL_REX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap()
});

pub static MOBILE_PHONE_REX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^1(3[0-9]|4[01456879]|5[0-35-9]|6[2567]|7[0-8]|8[0-9]|9[0-35-9])\d{8}$").unwrap()
});

pub static LANDLINE_PHONE_REX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(0\d{2,3})-?(\d{7,8})$").unwrap());

pub static CHINESE_NAME_REX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[\u4e00-\u9fa5]{2,4}$").unwrap());

pub static ID_CARD_REX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(^\d{15}$)|(^\d{18}$)|(^\d{17}(\d|X|x)$)").unwrap());

pub static REGISTER_WORD: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]+$").unwrap());

pub fn not_empty(str: &str) -> bool {
    !str.is_empty()
}

pub fn is_email(str: &str) -> bool {
    EMAIL_REX.is_match(str)
}

pub fn is_mobile_phone(str: &str) -> bool {
    MOBILE_PHONE_REX.is_match(str)
}

pub fn is_landline_phone(str: &str) -> bool {
    LANDLINE_PHONE_REX.is_match(str)
}

pub fn is_id_card(str: &str) -> bool {
    ID_CARD_REX.is_match(str)
}

pub fn is_chinese_name(str: &str) -> bool {
    CHINESE_NAME_REX.is_match(str)
}

pub fn is_register_word(str: &str) -> bool {
    REGISTER_WORD.is_match(str)
}

pub fn regex(pattern: &str, str: &str) -> bool {
    let regex = Regex::new(pattern);
    if let Ok(data) = regex {
        return data.is_match(str);
    }
    false
}
