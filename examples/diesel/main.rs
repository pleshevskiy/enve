#[macro_use]
extern crate diesel;

mod schema;

use diesel::prelude::*;
use dotenv::dotenv;

itconfig::config! {
    DATABASE_URL,
}

fn main() {
    dotenv().expect("dotenv setup to be successful");
    config::init();

    let connection = establish_connection();
    let posts = get_posts(&connection);

    println!("Displaying {} posts", posts.len());
    for post in posts {
        println!();
        println!("{}", post.title);
        println!("----------");
        println!("{}", post.body);
    }
}

fn get_posts(connection: &PgConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    posts
        .filter(published.eq(true))
        .limit(5)
        .get_results::<Post>(connection)
        .expect("Error loading posts")
}

fn establish_connection() -> PgConnection {
    let database_url = config::DATABASE_URL();
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
