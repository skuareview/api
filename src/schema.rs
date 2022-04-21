table! {
    agents (id) {
        id -> Int4,
        name -> Varchar,
        token -> Varchar,
    }
}

table! {
    lambdas (id) {
        id -> Int4,
        aws_lambda_region -> Varchar,
        aws_lambda_arn -> Varchar,
        aws_lambda_id -> Varchar,
    }
}

table! {
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

table! {
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
    }
}

table! {
    organizations (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    organizationsusers (id) {
        id -> Int4,
        id_organization -> Nullable<Int4>,
        id_user -> Nullable<Int4>,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        id_role -> Nullable<Int4>,
    }
}

joinable!(monitors -> lambdas (id_lambda));
joinable!(organizationsusers -> users (id_user));
joinable!(users -> roles (id_role));

allow_tables_to_appear_in_same_query!(
    agents,
    lambdas,
    metrics,
    monitors,
    organizations,
    organizationsusers,
    roles,
    users,
);
