use std::collections::HashMap;

use super::Response;
use actix_http::http::{
    header::{CacheDirective, ContentType},
    StatusCode,
};
use serde::Serialize;

/// Representation of a link to another resource
#[derive(Debug, Serialize)]
pub struct Link {
    pub href: String,
    pub templated: bool,
}

impl Link {
    pub fn new<S>(href: S, templated: bool) -> Self
    where
        S: Into<String>,
    {
        Self {
            href: href.into(),
            templated,
        }
    }
}
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Links {
    Single(Link),
    Multiple(Vec<Link>),
}

#[derive(Debug, Serialize)]
pub struct HalResponse<T>
where
    T: Serialize,
{
    #[serde(skip)]
    pub status: StatusCode,

    #[serde(skip)]
    pub cache_control: Vec<CacheDirective>,

    #[serde(flatten)]
    pub data: Option<T>,

    #[serde(rename = "_links", skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, Links>,
}

impl<T> HalResponse<T>
where
    T: Serialize,
{
    pub fn with_link<S>(mut self, key: S, link: Link) -> Self
    where
        S: Into<String>,
    {
        let key: String = key.into();

        let new_links = match self.links.remove(&key) {
            None => Links::Single(link),
            Some(Links::Single(existing)) => Links::Multiple(vec![existing, link]),
            Some(Links::Multiple(mut existing)) => {
                existing.push(link);
                Links::Multiple(existing)
            }
        };
        self.links.insert(key, new_links);

        self
    }
}

impl<T> Default for HalResponse<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            data: None,
            status: StatusCode::OK,
            cache_control: vec![CacheDirective::NoCache],
            links: HashMap::new(),
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl<I, O> From<I> for Response<HalResponse<O>>
where
    I: Into<HalResponse<O>>,
    O: Serialize,
{
    fn from(input: I) -> Self {
        let content_type = ContentType("application/hal+json".parse().unwrap());

        let hal_response: HalResponse<O> = input.into();
        Self {
            status: hal_response.status,
            cache_control: hal_response.cache_control.clone(),
            content_type,
            body: Some(hal_response),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};

    #[derive(Debug, Serialize, PartialEq)]
    struct TestResponseModel {}

    impl From<&str> for HalResponse<TestResponseModel> {
        fn from(_: &str) -> Self {
            Self { ..Self::default() }
        }
    }

    #[test]
    fn build_response() {
        let result: Response<HalResponse<TestResponseModel>> = "".into();

        check!(result.status == StatusCode::OK);
        check!(result.cache_control == vec![CacheDirective::NoCache]);
        check!(result.content_type == ContentType("application/hal+json".parse().unwrap()));

        let_assert!(Some(payload) = result.body);
        check!(payload.data == None);
        check!(payload.links.is_empty());
    }

    #[test]
    fn with_single_link() {
        let result = HalResponse::<TestResponseModel>::default()
            .with_link("self", Link::new("/self", false));

        check!(result.links.len() == 1);

        let_assert!(Some(self_links) = result.links.get("self"));
        let_assert!(Links::Single(self_link) = self_links);

        check!(self_link.href == "/self");
        check!(self_link.templated == false);
    }

    #[test]
    fn with_multiple_individual_link() {
        let result = HalResponse::<TestResponseModel>::default()
            .with_link("self", Link::new("/self", false))
            .with_link("other", Link::new("/other", false));

        check!(result.links.len() == 2);

        let_assert!(Some(self_links) = result.links.get("self"));
        let_assert!(Links::Single(self_link) = self_links);

        check!(self_link.href == "/self");
        check!(self_link.templated == false);

        let_assert!(Some(other_links) = result.links.get("other"));
        let_assert!(Links::Single(other_link) = other_links);

        check!(other_link.href == "/other");
        check!(other_link.templated == false);
    }

    #[test]
    fn with_multiple_same_links() {
        let result = HalResponse::<TestResponseModel>::default()
            .with_link("self", Link::new("/self", false))
            .with_link("self", Link::new("/other", false));

        check!(result.links.len() == 1);

        let_assert!(Some(self_links) = result.links.get("self"));
        let_assert!(Links::Multiple(self_link) = self_links);

        check!(self_link[0].href == "/self");
        check!(self_link[0].templated == false);

        check!(self_link[1].href == "/other");
        check!(self_link[1].templated == false);
    }
}
