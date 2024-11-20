use repo::dto::DtoList;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OneToOneDto<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> OneToOneDto<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OneToManyDto<O, M> {
    pub one: O,
    pub many: DtoList<M>,
}

impl<O, M> OneToManyDto<O, M> {
    pub fn new(one: O, many: DtoList<M>) -> Self {
        Self { one, many }
    }
}
