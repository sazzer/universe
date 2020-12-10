use crate::{
    authentication::ProviderID,
    http::{HalResponse, Response},
};

pub async fn list_providers() -> Response<HalResponse<()>> {
    let providers = vec![ProviderID::from("google"), ProviderID::from("twitter")];

    providers.into()
}
