use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The identity of some resource.
///
/// # Types
/// - `<I>` - The type to use for the ID.
#[derive(Debug, PartialEq)]
pub struct Identity<I> {
    /// The actual ID of the resource.
    pub id: I,
    /// The version of the resource.
    pub version: Uuid,
    /// The timestamp when the resource was created.
    pub created: DateTime<Utc>,
    /// The timestamp when the resource was last updated.
    pub updated: DateTime<Utc>,
}

/// The representation of some persisted data.
///
/// # Types
/// - `<I>` - The type to use for the ID.
/// - `<D>` - The type to use for the data.
#[derive(Debug, PartialEq)]
pub struct Model<I, D> {
    /// The identity of the resource.
    pub identity: Identity<I>,
    /// The data of the resource.
    pub data: D,
}

impl<I> Default for Identity<I>
where
    I: Default,
{
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: I::default(),
            version: Uuid::new_v4(),
            created: now,
            updated: now,
        }
    }
}
