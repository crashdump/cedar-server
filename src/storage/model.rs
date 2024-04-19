use diesel::{
    prelude::*,
    Identifiable, Insertable, Queryable, Selectable,
};
use uuid::Uuid;
use anyhow::{Context, Result};
use tonic::Status;
use tracing::info;
use crate::api::proto;

use crate::storage::schema::*;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(primary_key(store_id, id))]
#[diesel(belongs_to(Store))]
#[diesel(table_name = policies)]
pub struct Policy {
    pub id: Uuid,
    pub store_id: Uuid,
    pub description: String,
    pub statement: String,
}

impl Policy {
    pub fn get(conn: &mut PgConnection, store: &Store, id: &Uuid) -> Result<Policy> {
        policies::table
            .filter(policies::dsl::store_id.eq(store.id))
            .filter(policies::dsl::id.eq(id))
            .first(conn)
            .context(id)
    }

    pub fn list(conn: &mut PgConnection, store: &Store) -> Result<Vec<Policy>> {
        Policy::belonging_to(store)
            .load::<Policy>(conn)
            .context(format!("list for tenant {}", store.id))
    }

    pub fn delete(&self, conn: &mut PgConnection) -> Result<()> {
        diesel::delete(self)
            .execute(conn)
            .context(self.id.clone())?;
        Ok(())
    }

    pub fn into_proto(&self) -> proto::PolicyDefinition {
        proto::PolicyDefinition {
            id: self.id.braced().to_string(),
            description: self.description.clone(),
            statement: self.statement.clone(),
        }
    }
}


#[derive(Insertable)]
#[diesel(table_name = policies)]
pub struct NewPolicy<'a>  {
    pub id: &'a Uuid,
    pub store_id: &'a Uuid,
    pub description: &'a str,
    pub statement: &'a str,
}

impl<'a> NewPolicy<'a> {
    pub fn new(store: &Store, description: &'a str,  statement: &'a str) -> Self {
        let id = &Uuid::now_v7();
        info!("Creating policy with id: {}", id);
        NewPolicy {
            id,
            store_id: &store.id,
            description,
            statement,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> Result<Policy> {
        diesel::insert_into(policies::table)
            .values(self)
            .get_result(conn)
            .context(self.id.to_string())
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(primary_key(store_id, id))]
#[diesel(belongs_to(Store))]
#[diesel(table_name = entities)]
pub struct Entity {
    pub id: String,
    pub store_id: Uuid,
    pub type_: String,
}

/*
 *  Store
 */

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = stores)]
pub struct Store {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Store {
    pub fn get(conn: &mut PgConnection, id: &Uuid) -> Result<Store> {
        stores::table
            .find(id)
            .first(conn)
            .context(id)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Store>> {
        stores::dsl::stores
            .load::<Store>(conn)
            .context("list")
    }

    pub fn delete(&self, conn: &mut PgConnection) -> Result<()> {
        self.delete_entities(conn).and_then(|_| {
            diesel::delete(self)
                .execute(conn)
                .context(self.id.clone())
        })?;
        Ok(())
    }

    pub fn truncate(&self, conn: &mut PgConnection) -> Result<()> {
        self.delete_entities(conn)
    }
}

#[derive(Insertable)]
#[diesel(table_name = stores)]
pub struct NewStore<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub description: &'a str,
}

impl<'a> NewStore<'a> {
    pub fn new(name: &'a str, description: &'a str) -> Self {
        let id = &Uuid::now_v7();
        NewStore {
            id,
            name,
            description
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> Result<Store> {
        diesel::insert_into(stores::table)
            .values(self)
            .get_result(conn)
            .context(self.name.to_string())
    }
}