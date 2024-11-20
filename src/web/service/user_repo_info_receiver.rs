use std::sync::Arc;

use async_trait::async_trait;

use dto::user_repo_info_dto::{CreateUserRepoInfoDto, UserRepoInfoDto};


use crate::web::error::ApiResult;
use crate::web::service::UserRepoInfoServiceTrait;

pub struct UserRepoInfoReceiver {
    broker_receiver: Arc<dyn message_broker::Receiver<CreateUserRepoInfoDto>>,
    service: Arc<dyn UserRepoInfoServiceTrait>,
}

impl UserRepoInfoReceiver {
    pub fn new(
        broker_receiver: Arc<dyn message_broker::Receiver<CreateUserRepoInfoDto>>,
        service: Arc<dyn UserRepoInfoServiceTrait>,
    ) -> Self {
        Self {
            broker_receiver,
            service,
        }
    }
}

#[async_trait]
impl message_broker::Receiver<UserRepoInfoDto, ApiResult<UserRepoInfoDto>> for UserRepoInfoReceiver {
    async fn receive(&self) -> ApiResult<UserRepoInfoDto> {
        let info = self.broker_receiver.receive().await?;
        let dto = self.service.create(info).await?;
        Ok(dto)
    }
}