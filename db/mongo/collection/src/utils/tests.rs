use bson::doc;

use crate::utils::paginate_inmemory_collection;

#[test]
fn paginate_inmemory_collection_only_with_skip_success() {
    let numbers: Vec<u8> = (0..10).collect();
    let skip = 5;
    let pipeline = vec![doc! {"$skip": skip}];
    let desired_numbers: Vec<u8> = (5..10).collect();

    let paginated_numbers = paginate_inmemory_collection(numbers, pipeline);

    assert_eq!(paginated_numbers, desired_numbers)
}

#[test]
fn paginate_inmemory_collection_only_with_offset_success() {
    let numbers: Vec<u8> = (0..10).collect();
    let take = 5;
    let pipeline = vec![doc! {"$limit": take }];
    let desired_numbers: Vec<u8> = (0..5).collect();

    let paginated_numbers = paginate_inmemory_collection(numbers, pipeline);

    assert_eq!(paginated_numbers, desired_numbers)
}

#[test]
fn paginate_inmemory_collection_with_offset_and_limit_success() {
    let numbers: Vec<u8> = (0..10).collect();
    let skip = 5;
    let take = 3;
    let pipeline = vec![doc! {"$skip": skip}, doc! {"$limit": take }];
    let desired_numbers: Vec<u8> = (5..8).collect();

    let paginated_numbers = paginate_inmemory_collection(numbers, pipeline);

    assert_eq!(paginated_numbers, desired_numbers)
}

#[test]
fn paginate_inmemory_collection_and_get_all_success() {
    let numbers: Vec<u8> = (0..10).collect();
    let pipeline = vec![];
    let desired_numbers: Vec<u8> = (0..10).collect();

    let paginated_numbers = paginate_inmemory_collection(numbers, pipeline);

    assert_eq!(paginated_numbers, desired_numbers)
}

#[test]
fn paginate_inmemory_collection_out_of_bound_success() {
    let numbers: Vec<u8> = (0..10).collect();
    let skip = 9;
    let take = 2;
    let pipeline = vec![doc! {"$skip": skip}, doc! {"$limit": take }];
    let desired_numbers: Vec<u8> = (9..=9).collect();

    let paginated_numbers = paginate_inmemory_collection(numbers, pipeline);

    assert_eq!(paginated_numbers, desired_numbers)
}
