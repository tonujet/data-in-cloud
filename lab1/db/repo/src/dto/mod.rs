use serde::{Deserialize, Serialize};

pub mod repository_dto;

#[derive(Serialize, PartialEq, Deserialize, Debug)]
pub struct ListResponse<T> {
    pub dtos: Vec<T>,
    pub count: u64,
    pub last_taken_entity_number: Option<u64>,
}

impl<T> ListResponse<T> {
    pub fn new(dtos: Vec<T>, count: u64, take: Option<u64>, offset: Option<u64>) -> Self {
        let last_taken_entity_number = match (take, offset) {
            (None, None) => Some(count),
            (None, Some(offset)) => {
                if offset >= count {
                    None
                } else {
                    Some(count)
                }
            }

            (Some(take), Some(offset)) => {
                let last_taken_number = take + offset;
                if offset > count {
                    None
                } else if last_taken_number > count {
                    Some(count)
                } else {
                    Some(last_taken_number)
                }
            }

            (Some(take), None) => {
                if take > count {
                    Some(count)
                } else {
                    Some(take)
                }
            }
        };
        Self {
            dtos,
            count,
            last_taken_entity_number,
        }
    }
}
