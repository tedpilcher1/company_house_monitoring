// @generated automatically by Diesel CLI.

diesel::table! {
    company (company_house_id) {
        company_house_id -> Uuid,
        first_monitored_at -> Timestamp,
    }
}

diesel::table! {
    company_snapshot (id) {
        id -> Uuid,
        company_house_id -> Text,
        snapshot_data -> Jsonb,
    }
}

diesel::table! {
    notable_change (id) {
        id -> Uuid,
        subscription_id -> Uuid,
        field -> Text,
    }
}

diesel::table! {
    subscription (id) {
        id -> Uuid,
        company_house_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    company,
    company_snapshot,
    notable_change,
    subscription,
);
