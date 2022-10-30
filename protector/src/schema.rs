table! {
    phone_report (id) {
        id -> Int4,
        phone_number -> Varchar,
        origin_name -> Text,
        count_confirm -> Int4,
        count_report -> Int4,
        date_created -> Varchar,
        date_modified -> Varchar,
    }
}

// table! {
//     phone_search (id) {
//         id -> Int4,
//         phone_number -> Varchar,
//         count_search -> Int4,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }
//
// table! {
//     phone_comment (id) {
//         id -> Int4,
//         phone_id -> Int4,
//         name_user -> Varchar,
//         body_comment -> Text,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }