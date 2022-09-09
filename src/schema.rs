// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Int4,
        name -> Varchar,
        token -> Varchar,
    }
}

diesel::table! {
    email_confirmations (id) {
        id -> Int4,
        code -> Int4,
        expiration_date -> Date,
    }
}

diesel::table! {
    lambdas (id) {
        id -> Int4,
        aws_lambda_region -> Varchar,
        aws_lambda_arn -> Varchar,
        aws_lambda_id -> Varchar,
    }
}

diesel::table! {
    metrics (id) {
        id -> Int4,
        load_average_1 -> Nullable<Varchar>,
        load_average_2 -> Nullable<Varchar>,
        load_average_3 -> Nullable<Varchar>,
        memory_used -> Nullable<Varchar>,
        memory_total -> Nullable<Varchar>,
        cpu_temp -> Nullable<Varchar>,
        cpu_load -> Nullable<Varchar>,
        id_agent -> Nullable<Int4>,
    }
}

diesel::table! {
    monitors (id) {
        id -> Int4,
        name -> Varchar,
        aws_eventbridge_region -> Varchar,
        aws_eventbridge_name -> Varchar,
        aws_eventbridge_description -> Varchar,
        aws_eventbridge_event_bus_name -> Varchar,
        aws_eventbridge_schedule_expression -> Varchar,
        id_agent -> Nullable<Int4>,
        id_lambda -> Nullable<Int4>,
        id_organization -> Nullable<Int4>,
        id_user -> Nullable<Uuid>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    organizations_users (id) {
        id -> Int4,
        id_organization -> Nullable<Int4>,
        id_user -> Nullable<Uuid>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        id_role -> Nullable<Int4>,
    }
}

diesel::table! {
    users_email_confirmations (id) {
        id -> Int4,
        id_user -> Nullable<Uuid>,
        id_email_confirmation -> Nullable<Int4>,
    }
}

diesel::joinable!(monitors -> lambdas (id_lambda));
diesel::joinable!(users -> roles (id_role));
diesel::joinable!(users_email_confirmations -> email_confirmations (id_email_confirmation));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    email_confirmations,
    lambdas,
    metrics,
    monitors,
    organizations,
    organizations_users,
    roles,
    users,
    users_email_confirmations,
);
