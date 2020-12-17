use actix_http::http::{
    header::{CacheDirective, ContentType, EntityTag},
    StatusCode,
};
use serde::Serialize;
use serde_json::Value;

use super::Response;

/// Links represent navigational transitions.
#[derive(Debug, Serialize)]
pub struct Link {
    /// The URI of the linked resource
    pub href: String,
    /// Defines the relationship of the link to its entity
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rel: Vec<String>,
    /// Describes aspects of the link based on the current representation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<String>,
}

impl Link {
    /// Construct a new link
    ///
    /// # Parameters
    /// - `href` - The href for the link
    pub fn new<S>(href: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            href: href.into(),
            rel: vec![],
            class: vec![],
        }
    }

    /// Add a new link relation to the link
    ///
    /// # Parameters
    /// - `rel` - The link relation to add
    pub fn with_rel<S>(mut self, rel: S) -> Self
    where
        S: Into<String>,
    {
        self.rel.push(rel.into());
        self
    }

    /// Add a new class to the link
    ///
    /// # Parameters
    /// - `class` - The class to add
    pub fn with_class<S>(mut self, class: S) -> Self
    where
        S: Into<String>,
    {
        self.class.push(class.into());
        self
    }
}

/// Sub-entities can be expressed as either an embedded link or an embedded representation.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Entity {
    Link {
        /// Defines the relationship of the sub-entity to its parent
        #[serde(skip_serializing_if = "Vec::is_empty")]
        rel: Vec<String>,
        /// Describes the nature of an entity's content based on the current representation
        #[serde(skip_serializing_if = "Vec::is_empty")]
        class: Vec<String>,
        /// The URI of the linked sub-entity
        href: String,
    },
    Embedded {
        /// Defines the relationship of the sub-entity to its parent
        #[serde(skip_serializing_if = "Vec::is_empty")]
        rel: Vec<String>,
        /// Describes the nature of an entity's content based on the current representation
        #[serde(skip_serializing_if = "Vec::is_empty")]
        class: Vec<String>,
        /// A set of key-value pairs that describe the state of an entity
        properties: Value,
        /// A collection of items that describe navigational links, distinct from entity relationships
        #[serde(skip_serializing_if = "Vec::is_empty")]
        links: Vec<Link>,
    },
}

impl Entity {
    /// Construct a new link entity
    ///
    /// # Parameters
    /// - `href` - The href for the link
    pub fn new_link<S>(href: S) -> Self
    where
        S: Into<String>,
    {
        Self::Link {
            href: href.into(),
            rel: vec![],
            class: vec![],
        }
    }

    /// Construct a new embedded entity
    ///
    /// # Parameters
    /// - `properties` - The proprties for the entity
    pub fn new_embedded<S>(properties: S) -> Self
    where
        S: Serialize,
    {
        let serialized = serde_json::to_value(properties).unwrap();
        Self::Embedded {
            properties: serialized,
            rel: vec![],
            class: vec![],
            links: vec![],
        }
    }

    /// Add a new relation to the entity
    ///
    /// # Parameters
    /// - `value` - The relation to add
    pub fn with_rel<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        match self {
            Self::Link {
                href,
                mut rel,
                class,
            } => {
                rel.push(value.into());

                Self::Link { href, rel, class }
            }
            Self::Embedded {
                properties,
                mut rel,
                class,
                links,
            } => {
                rel.push(value.into());

                Self::Embedded {
                    properties,
                    rel,
                    class,
                    links,
                }
            }
        }
    }

    /// Add a new class to the entity
    ///
    /// # Parameters
    /// - `value` - The class to add
    pub fn with_class<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        match self {
            Self::Link {
                href,
                rel,
                mut class,
            } => {
                class.push(value.into());

                Self::Link { href, rel, class }
            }
            Self::Embedded {
                properties,
                rel,
                mut class,
                links,
            } => {
                class.push(value.into());

                Self::Embedded {
                    properties,
                    rel,
                    class,
                    links,
                }
            }
        }
    }

    /// Add a new link to the entity
    ///
    /// # Parameters
    /// - `value` - The link to add
    pub fn with_link(self, value: Link) -> Self {
        match self {
            Self::Embedded {
                properties,
                rel,
                class,
                mut links,
            } => {
                links.push(value);

                Self::Embedded {
                    properties,
                    rel,
                    class,
                    links,
                }
            }
            _ => unimplemented!(),
        }
    }
}

/// Wrapper around the value for a field
#[derive(Debug, Serialize)]
pub struct FieldValue(String);

impl FieldValue {
    /// Create a new instance of `FieldValue`.
    ///
    /// # Parameters
    /// - `value` - The value to wrap
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self(value.into())
    }
}

/// Trait that can be implemented by anything that can be converted into a `FieldValue`.
pub trait IntoFieldValue {
    /// Convert the value into a `FieldValue` object
    fn into_field_value(self) -> FieldValue;
}

impl<S> IntoFieldValue for S
where
    S: Into<String>,
{
    fn into_field_value(self) -> FieldValue {
        FieldValue::new(self)
    }
}

/// Fields represent controls inside of actions
#[derive(Debug, Serialize)]
pub struct Field {
    /// A name describing the control
    pub name: String,
    /// Describes aspects of the field based on the current representation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<String>,
    /// The input type of the field
    pub r#type: String,
    /// A value assigned to the field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<FieldValue>,
}

impl Field {
    /// Create a new field
    ///
    /// # Parameters
    /// - `name` - The name of the field
    /// - `type` - The type of the field
    pub fn new<N, T>(name: N, r#type: T) -> Self
    where
        N: Into<String>,
        T: Into<String>,
    {
        Self {
            name: name.into(),
            r#type: r#type.into(),
            class: vec![],
            value: None,
        }
    }

    /// Add a new class to the field
    ///
    /// # Parameters
    /// - `class` - The class to add
    pub fn with_class<S>(mut self, class: S) -> Self
    where
        S: Into<String>,
    {
        self.class.push(class.into());
        self
    }

    /// Provide a value for the field
    ///
    /// # Parameters
    /// - `value` - The new value to use
    pub fn with_value<S>(mut self, value: S) -> Self
    where
        S: IntoFieldValue,
    {
        self.value = Some(value.into_field_value());
        self
    }
}

/// Actions show available behaviors an entity exposes.
#[derive(Debug, Serialize)]
pub struct Action {
    /// A string that identifies the action to be performed
    pub name: String,
    /// Describes the nature of an action based on the current representation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<String>,
    /// An enumerated attribute mapping to a protocol method
    pub method: String,
    /// The URI of the action
    pub href: String,
    pub fields: Vec<Field>,
    /// The encoding type for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl Action {
    /// Create a new action
    ///
    /// # Parameters
    /// - `name` - The name of the action
    /// - `method` - The method to use for the action
    /// - `href` - The URI of the action
    pub fn new<N, M, H>(name: N, method: M, href: H) -> Self
    where
        N: Into<String>,
        M: Into<String>,
        H: Into<String>,
    {
        Self {
            name: name.into(),
            method: method.into(),
            href: href.into(),
            class: vec![],
            fields: vec![],
            r#type: None,
        }
    }

    /// Add a new class to the action
    ///
    /// # Parameters
    /// - `class` - The class to add
    pub fn with_class<S>(mut self, class: S) -> Self
    where
        S: Into<String>,
    {
        self.class.push(class.into());
        self
    }

    /// Provide a new field for the action
    ///
    /// # Parameters
    /// - `value` - The new value to use
    pub fn with_field(mut self, value: Field) -> Self {
        self.fields.push(value);
        self
    }

    /// Provide a request content type for the action
    ///
    /// # Parameters
    /// - `value` - The new value to use
    pub fn with_type<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.r#type = Some(value.into());
        self
    }

    /// Helper to indicate that the request content type should be JSON
    pub fn with_type_json(self) -> Self {
        self.with_type("application/json")
    }

    /// Helper to indicate that the request content type should be Form encoded
    pub fn with_type_form(self) -> Self {
        self.with_type("application/x-www-form-urlencoded")
    }
}

/// The actual representation of a Siren document.
#[derive(Debug, Serialize)]
pub struct SirenPayload<T>
where
    T: Serialize,
{
    /// Describes the nature of an entity's content based on the current representation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<String>,
    /// A set of key-value pairs that describe the state of an entity
    pub properties: T,
    /// A collection of related sub-entities
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,
    /// A collection of items that describe navigational links, distinct from entity relationships
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    /// A collection of action objects
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
}

impl<T> SirenPayload<T>
where
    T: Serialize,
{
    /// Create a new Siren response payload
    ///
    /// # Parameters
    /// - `properties` - The properties of the response payload
    pub fn new(properties: T) -> Self {
        Self {
            properties,
            class: vec![],
            entities: vec![],
            links: vec![],
            actions: vec![],
        }
    }

    /// Add a new class to the response payload
    ///
    /// # Parameters
    /// - `class` - The class to add
    pub fn with_class<S>(mut self, class: S) -> Self
    where
        S: Into<String>,
    {
        self.class.push(class.into());
        self
    }

    /// Add a new entity to the response payload
    ///
    /// # Parameters
    /// - `entity` - The entity to add
    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    /// Add a new link to the response payload
    ///
    /// # Parameters
    /// - `link` - The link to add
    pub fn with_link(mut self, link: Link) -> Self {
        self.links.push(link);
        self
    }

    /// Add a new action to the response payload
    ///
    /// # Parameters
    /// - `action` - The action to add
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

/// Wrapper around a Siren payload to add other HTTP response details
#[derive(Debug, Serialize)]
pub struct SirenResponse<T>
where
    T: Serialize,
{
    /// The body of the response
    pub body: Option<SirenPayload<T>>,

    /// The status code
    #[serde(skip)]
    pub status: StatusCode,

    /// The caching controls
    #[serde(skip)]
    pub cache_control: Vec<CacheDirective>,

    /// The etag
    #[serde(skip)]
    pub etag: Option<EntityTag>,
}

impl<T> Default for SirenResponse<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            body: None,
            status: StatusCode::OK,
            cache_control: vec![CacheDirective::NoCache],
            etag: None,
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl<I, O> From<I> for Response<SirenPayload<O>>
where
    I: Into<SirenResponse<O>>,
    O: Serialize,
{
    fn from(input: I) -> Self {
        let content_type = ContentType("application/vnd.siren+json".parse().unwrap());

        let siren_response: SirenResponse<O> = input.into();
        Self {
            content_type,
            body: siren_response.body,
            status: siren_response.status,
            cache_control: siren_response.cache_control,
            etag: siren_response.etag,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn build_api_example() {
        let response = SirenPayload::new(json!({
          "orderNumber": 42,
          "itemCount": 3,
          "status": "pending"
        }))
        .with_class("order")
        .with_entity(
            Entity::new_link("http://api.x.io/orders/42/items")
                .with_class("items")
                .with_class("collection")
                .with_rel("http://x.io/rels/order-items"),
        )
        .with_entity(
            Entity::new_embedded(json!({
              "customerId": "pj123",
              "name": "Peter Joseph"
            }))
            .with_class("info")
            .with_class("customer")
            .with_rel("http://x.io/rels/customer")
            .with_link(Link::new("http://api.x.io/customers/pj123").with_rel("self")),
        )
        .with_action(
            Action::new("add-item", "POST", "http://api.x.io/orders/42/items")
                .with_type_form()
                .with_field(Field::new("orderNumber", "hidden").with_value("42"))
                .with_field(Field::new("productCode", "text"))
                .with_field(Field::new("quantity", "number")),
        )
        .with_link(Link::new("http://api.x.io/orders/42").with_rel("self"))
        .with_link(Link::new("http://api.x.io/orders/41").with_rel("previous"))
        .with_link(Link::new("http://api.x.io/orders/43").with_rel("next"));

        let json = serde_json::to_value(response).unwrap();

        insta::assert_json_snapshot!(json, @r###"
        {
          "class": [
            "order"
          ],
          "properties": {
            "orderNumber": 42,
            "itemCount": 3,
            "status": "pending"
          },
          "entities": [
            {
              "rel": [
                "http://x.io/rels/order-items"
              ],
              "class": [
                "items",
                "collection"
              ],
              "href": "http://api.x.io/orders/42/items"
            },
            {
              "rel": [
                "http://x.io/rels/customer"
              ],
              "class": [
                "info",
                "customer"
              ],
              "properties": {
                "customerId": "pj123",
                "name": "Peter Joseph"
              },
              "links": [
                {
                  "href": "http://api.x.io/customers/pj123",
                  "rel": [
                    "self"
                  ]
                }
              ]
            }
          ],
          "links": [
            {
              "href": "http://api.x.io/orders/42",
              "rel": [
                "self"
              ]
            },
            {
              "href": "http://api.x.io/orders/41",
              "rel": [
                "previous"
              ]
            },
            {
              "href": "http://api.x.io/orders/43",
              "rel": [
                "next"
              ]
            }
          ],
          "actions": [
            {
              "name": "add-item",
              "method": "POST",
              "href": "http://api.x.io/orders/42/items",
              "fields": [
                {
                  "name": "orderNumber",
                  "type": "hidden",
                  "value": "42"
                },
                {
                  "name": "productCode",
                  "type": "text"
                },
                {
                  "name": "quantity",
                  "type": "number"
                }
              ],
              "type": "application/x-www-form-urlencoded"
            }
          ]
        }
        "###);
    }
}
