// @generated automatically by Diesel CLI.

diesel::table! {
    stores (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    entities (store_id, id) {
        store_id -> Uuid,
        #[max_length = 64]
        id -> Varchar,
        #[sql_name = "type"]
        #[max_length = 64]
        type_ -> Varchar,
    }
}

diesel::table! {
    policies (store_id, id) {
        id -> Uuid,
        store_id -> Uuid,
        #[max_length = 64]
        description -> Varchar,
        statement -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(entities, policies,);


