use diesel::{prelude::*};
use seed_rocket_diesel_template::schema;

type Result<T> = std::result::Result<T, diesel::result::Error>;

pub fn create_connection() -> SqliteConnection {
    let db_location: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    SqliteConnection::establish(&db_location)
        .unwrap_or_else(|_| panic!("Cannot connect to database at '{}'", db_location))
}

pub mod student {
    use super::*;
    use seed_rocket_diesel_template::student::*;

    pub fn insert(conn: &SqliteConnection, item: NewStudent) -> Result<usize> {
        diesel::insert_into(schema::student::table)
            .values(&item)
            .execute(conn)
    }

    pub fn select(conn: &SqliteConnection) -> Result<Vec<Student>> {
        schema::student::table.load(conn)
    }

    pub fn update(conn: &SqliteConnection, item: Student) -> Result<usize> {
        diesel::update(schema::student::table)
            .set(&item)
            .execute(conn)
    }

    pub fn delete(conn: &SqliteConnection, item: Student) -> Result<usize> {
        use schema::student::dsl::*;
        //diesel::delete(student.find(item.id)).execute(conn)
        diesel::delete(student.filter(id.eq(item.id))).execute(conn)
    }
}
