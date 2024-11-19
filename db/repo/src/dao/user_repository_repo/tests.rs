use uuid::Uuid;

use crate::dao::BlobConnRepoTrait;
use crate::utils::user_repo::user_repo_test_helper;

#[tokio::test]
async fn add_repo_success() {
    let (repo, user_id, repo_id) = user_repo_test_helper::get_mock_repo_with_starter();
    
    let res = repo.add_pair(&user_id, &repo_id).await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn add_repo_duplicate_failure() {
    let (repo, user_id, repo_id) = user_repo_test_helper::get_mock_repo_with_starter();

    let _ = repo.add_pair(&user_id, &repo_id).await;
    let res = repo.add_pair(&user_id, &repo_id).await;

    assert!(res.is_err())
}

#[tokio::test]
async fn delete_repo_success() {
    let (repo, user_id, repo_id) = user_repo_test_helper::get_mock_repo_with_starter();

    let _ = repo.add_pair(&user_id, &repo_id).await;
    let res = repo.delete_pair(&user_id, &repo_id).await;

    assert!(res.is_ok())
}

#[tokio::test]
async fn delete_nonexisting_repo() {
    let (repo, user_id, repo_id) = user_repo_test_helper::get_mock_repo_with_starter();

    let res = repo.delete_pair(&user_id, &repo_id).await;

    assert!(res.is_err())
}

#[tokio::test]
async fn list_repos_success() {
    let (repo, user_id, repo_id1) = user_repo_test_helper::get_mock_repo_with_starter();
    let repo_id2 = Uuid::new_v4();
    let repo_id3 = Uuid::new_v4();
    let expected_ids = vec![repo_id1, repo_id2, repo_id3];
    let repo_ids = expected_ids.iter().rev().collect();

    
    let _ = repo.add_pairs(&user_id, repo_ids).await;
    let res = repo.list_pairs(&user_id).await;
    
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected_ids);
    
}

#[tokio::test]
async fn list_repos_after_delete_success() {
    let (repo, user_id, repo_id1) = user_repo_test_helper::get_mock_repo_with_starter();
    let repo_id2 = Uuid::new_v4();
    let repo_id3 = Uuid::new_v4();
    let repo_ids = vec![&repo_id1, &repo_id2, &repo_id3];
    let expected_ids = vec![repo_id3]; 
    
    let _ = repo.add_pairs(&user_id, repo_ids).await;
    let _ = repo.delete_pair(&user_id, &repo_id1).await;
    let _ = repo.delete_pair(&user_id, &repo_id2).await;
    let res = repo.list_pairs(&user_id).await;
    
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected_ids);
}