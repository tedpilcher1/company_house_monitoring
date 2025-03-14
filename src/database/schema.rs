diesel::table! {
    company (company_house_id) {
        company_house_id -> Uuid,
        first_monitored_at -> Timestamp,
    }
}

diesel::table! {
    subscription (id) {
        id -> Uuid,
        company_house_id -> Text,
        created_at -> Timestamp,
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
    company_snapshot (id) {
        id -> Uuid,
        company_house_id -> Text,
        snapshot_data -> Jsonb,
    }
}
