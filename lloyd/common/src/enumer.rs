use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnumDesc {
    pub id: i32,
    pub name: String,
    pub note: String,
}

pub trait EnumFrom<T>: Sized {
    fn enum_by(value: T) -> Option<Self>;
}
