use crate::{
    http::{HalResponse, Response},
    model::Identity,
    users::{UserData, UserModel},
};

use super::model::UserResponse;

pub async fn get_user() -> Response<HalResponse<UserResponse>> {
    let user: UserModel = UserModel {
        identity: Identity::default(),
        data: UserData {
            display_name: "Graham".to_owned(),
            email: None,
            username: None,
            authentications: vec![],
        },
    };

    user.into()
}
