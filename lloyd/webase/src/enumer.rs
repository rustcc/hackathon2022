use common::enumer::EnumDesc;
use common::enumer::EnumFrom;
use procmac::EnumFrom;

#[derive(Debug, EnumFrom)]
pub enum Whether {
    #[note("是")]
    Yes = 1,

    #[note("否")]
    No = 0,
}

#[derive(Debug, EnumFrom)]
pub enum Sex {
    #[note("男")]
    Man = 1,

    #[note("女")]
    Woman = 2,

    #[note("未知")]
    Unknow = 3,
}
