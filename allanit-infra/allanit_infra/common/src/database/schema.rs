// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int8,
        external_id -> Text,
        name -> Text,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        org_no -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    emails_inbound (id) {
        id -> Int8,
        message_id -> Text,
        from_addr -> Text,
        to_addr -> Array<Nullable<Text>>,
        subject -> Nullable<Text>,
        received_at -> Timestamptz,
        raw_source -> Nullable<Text>,
        parsed -> Nullable<Jsonb>,
        status -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    invoices_raw (id) {
        id -> Int8,
        source_system -> Text,
        external_id -> Nullable<Text>,
        customer_external_id -> Nullable<Text>,
        email_id -> Nullable<Int8>,
        payload -> Jsonb,
        file_uri -> Nullable<Text>,
        ocr_text -> Nullable<Text>,
        status -> Text,
        received_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    logs (id) {
        id -> Int4,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        level -> Varchar,
        module -> Varchar,
        action -> Varchar,
        custom_msg -> Nullable<Text>,
    }
}

diesel::table! {
    purchase_orders (id) {
        id -> Int8,
        external_id -> Text,
        customer_id -> Nullable<Int8>,
        status -> Text,
        description -> Nullable<Text>,
        amount -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    system_logs (id) {
        id -> Int8,
        ts -> Timestamptz,
        module -> Text,
        level -> Text,
        action -> Text,
        message -> Nullable<Text>,
        meta -> Nullable<Jsonb>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(invoices_raw -> emails_inbound (email_id));
diesel::joinable!(purchase_orders -> customers (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    emails_inbound,
    invoices_raw,
    logs,
    purchase_orders,
    system_logs,
    users,
);
