use actix_http::http::header::CacheDirective;

use crate::{
    authentication::ProviderID,
    http::{HalResponse, Link},
};

impl From<Vec<ProviderID>> for HalResponse<()> {
    fn from(providers: Vec<ProviderID>) -> Self {
        let mut result = Self {
            cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
            ..Self::default()
        }
        .with_link("self", Link::new("/authentication"));

        for provider in providers {
            result = result.with_link("item", provider.into());
        }

        result
    }
}
