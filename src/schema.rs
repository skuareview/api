table! {
    agents (id) {
        id -> Int4,
        name -> Varchar,
        token -> Varchar,
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
    }
}

allow_tables_to_appear_in_same_query!(
    agents,
    metrics,
);
