use crate::{models::*, schema::*};
use diesel::prelude::*;
use diesel::PgConnection;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find_multiple(conn: &PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id)
            .load::<Rustacean>(conn)
    }

    pub fn find(conn: &PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).first::<Rustacean>(conn)
    }

    pub fn create(conn: &PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(conn)
    }

    pub fn save(conn: &PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(rustacean.email),
                rustaceans::name.eq(rustacean.name),
            ))
            .execute(conn)?;
        Self::find(conn, id)
    }

    pub fn delete(conn: &PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(conn)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub fn find_multiple(conn: &PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id)
            .load::<Crate>(conn)
    }

    pub fn find(conn: &PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).first::<Crate>(conn)
    }

    pub fn create(conn: &PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(conn)
    }

    pub fn save(conn: &PgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::name.eq(a_crate.name),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .execute(conn)?;
        Self::find(conn, id)
    }

    pub fn delete(conn: &PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn)
    }
}
