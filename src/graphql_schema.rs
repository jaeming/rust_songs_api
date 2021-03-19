extern crate diesel;
extern crate dotenv;
// use crate::schema::songs;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::{EmptyMutation, RootNode};
use std::env;

#[derive(Queryable)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub duration: i32,
    pub artist_id: i32,
}

#[derive(Queryable)]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

#[juniper::object(description = "A Song")]
impl Song {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn duration(&self) -> i32 {
        self.duration
    }
    pub fn artist(&self) -> Artist {
        use crate::schema::artists::dsl::*;
        let connection = establish_connection();
        artists
            .find(self.artist_id)  
            .first::<Artist>(&connection)
            .unwrap()            
    }
}

#[juniper::object(description = "An Artist")]
impl Artist {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn songs(&self) -> Vec<Song> {
        use crate::schema::songs::dsl::*;
        let connection = establish_connection();
        songs
            .filter(artist_id.eq(self.id))
            .load::<Song>(&connection)
            .expect("Error loading songs")
    }
}

pub struct QueryRoot;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[juniper::object]
impl QueryRoot {
    fn songs() -> Vec<Song> {
        use crate::schema::songs::dsl::*;
        let connection = establish_connection();
        songs
            .load::<Song>(&connection)
            .expect("Error loading songs")
    }

    fn artists() -> Vec<Artist> {
        use crate::schema::artists::dsl::*;
        let connection = establish_connection();
        artists
            .load::<Artist>(&connection)
            .expect("Error loading artists")
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
