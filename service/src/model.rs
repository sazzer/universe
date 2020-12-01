use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The identity of some resource.
#[derive(Debug)]
pub struct Identity<I> {
    pub id: I,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// The representation of some persisted data.
#[derive(Debug)]
pub struct Model<I, D> {
    pub identity: Identity<I>,
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
