use crate::media::MediaTrait;
use crate::schema::movie;
use crate::streamable_media::{StreamableMedia, StreamableTrait};
use crate::DatabaseError;

use async_trait::async_trait;
use tokio_diesel::*;

/// Struct represents a Movie entry in the database
#[derive(Clone, Identifiable, Queryable, Associations)]
#[belongs_to(StreamableMedia, foreign_key = "id")]
#[table_name = "movie"]
pub struct Movie {
    /// id of the movie that is also a foreign key to a media entry.
    id: i64,
}

/// Struct reperesents a insertable movie entry
#[derive(Clone, Insertable)]
#[table_name = "movie"]
pub struct InsertableMovie {
    /// id of a media entry that should be used as a foreign key.
    id: i64,
}

#[async_trait]
impl StreamableTrait for InsertableMovie {
    /// Method returns a new instance of InsertableMovie, this is a trait method because it is used
    /// to indicate that this specific media entry can be streamed.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of the movie we are inserting, this id should already exist in the media table.
    fn new(id: i64) -> Self {
        Self { id }
    }

    /// Method inserts the object into the table movie returning its id which should be equivalent
    /// to the field id.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    async fn insert(&self, conn: &crate::DbConnection) -> Result<i64, DatabaseError> {
        diesel::insert_into(movie::table)
            .values(self.clone())
            .execute_async(conn)
            .await?;

        Ok(self.id)
    }
}

impl MediaTrait for InsertableMovie {}
