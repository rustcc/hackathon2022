use crate::doc::model::ApiModelTrait;
use common::doc::ApiField;
use common::doc::ApiFieldType;
use common::doc::ApiMember;
use common::doc::ApiModel;
use common::validator::ValidateError;
use common::validator::Validator;
use procmac::ApiModel;
use procmac::Validator;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiModel, Validator)]
pub struct ReqPage<T: ApiModelTrait + Validator> {
    #[note("第几页, 页码从0开始")]
    pub page: u64,

    #[note("每页大小")]
    pub size: u64,

    #[note("查询条件")]
    pub condition: T,
}

#[derive(Debug, Serialize, Deserialize, ApiModel)]
pub struct ResPage<T: ApiModelTrait> {
    #[note("总行数")]
    pub rows: u64,

    #[note("总页数")]
    pub pages: u64,

    #[note("总记录")]
    pub records: Vec<T>,
}

impl<T: ApiModelTrait> ResPage<T> {
    pub fn new(rows: u64, size: u64, records: Vec<T>) -> Self {
        Self {
            rows,
            pages: rows / size,
            records,
        }
    }

    pub fn default() -> Self {
        Self {
            rows: 0,
            pages: 0,
            records: vec![],
        }
    }
}
