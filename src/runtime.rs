use crate::web::state::AppState;
use std::sync::Arc;

pub fn run_detached_tasks(state: &AppState) {
    let user_repo_info_receiver = Arc::clone(&state.user_repo_info_state.receiver);

    tokio::spawn(async move {
        loop {
            let res = user_repo_info_receiver.receive().await;
            if let Err(e) = res {
                eprintln!("{e}");
                break;
            }
        }
    });
}
