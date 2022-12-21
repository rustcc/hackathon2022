use crate::doc::prelude::*;
use procmac::ApiModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiModel)]
pub struct DeleteRes {
    #[note("影响的行数")]
    pub affected: u64,
}

#[derive(Debug, Serialize, Deserialize, ApiModel)]
pub struct CreateRes {
    #[note("主键")]
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, ApiModel)]
pub struct UpdateRes {
    #[note("主键")]
    pub id: i64,
}
