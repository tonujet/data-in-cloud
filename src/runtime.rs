use crate::web::state::AppState;

pub fn run_detached_tasks(state: AppState) {
    let user_repo_info_receiver = state.user_repo_info_state.receiver;

    tokio::spawn(async move {
        loop {
            let res = user_repo_info_receiver.receive().await;
            if let Err(e) = res {
                println!("{e}");
                break;
            }
        }
    });
}
