use bson::Document;

#[cfg(test)]
mod tests;

pub fn paginate_inmemory_collection<E>(entities: Vec<E>, pipeline: Vec<Document>) -> Vec<E> {
    let mut skip: Option<usize> = None;
    let mut limit: Option<usize> = None;

    for doc in pipeline {
        let poss_skip = doc.get("$skip");
        let poss_limit = doc.get("$limit");

        if let (None, Some(poss_skip)) = (skip, poss_skip) {
            skip = Some(poss_skip.as_i32().unwrap() as usize)
        }

        if let (None, Some(poss_limit)) = (limit, poss_limit) {
            limit = Some(poss_limit.as_i32().unwrap() as usize)
        }
    }

    let skip = skip.unwrap_or(0);
    let limit = limit.unwrap_or(entities.len());

    entities.into_iter().skip(skip).take(limit).collect()
}
