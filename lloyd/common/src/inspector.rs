use tools::validate::is_register_word;

// 检查器
pub trait Inspector<T> {
    // 校验是否通过
    fn validate(data: &T) -> bool;

    // 校验规则描述
    fn desc() -> String;
}

pub struct Word<const MIN: usize, const MAX: usize>;
impl<const MIN: usize, const MAX: usize> Inspector<String> for Word<MIN, MAX> {
    fn validate(data: &String) -> bool {
        if data.len() > MAX {
            return false;
        }
        if data.len() < MIN {
            return false;
        }
        is_register_word(data)
    }

    fn desc() -> String {
        format!("^[a-zA-Z][a-zA-Z0-9_]{{{}{}}}$", MIN, MAX)
    }
}

pub type RegisterWord = Word<1, 30>;
