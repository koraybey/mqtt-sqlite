// @generated automatically by Diesel CLI.

diesel::table! {
    logs (id) {
        id -> Int4,
        #[max_length = 255]
        friendly_name -> Nullable<Varchar>,
        current -> Nullable<Float8>,
        energy -> Nullable<Float8>,
        power -> Nullable<Float8>,
        last_seen -> Nullable<Timestamp>,
        voltage -> Nullable<Int4>,
        linkquality -> Nullable<Int4>,
        #[max_length = 255]
        state -> Nullable<Varchar>,
        contact -> Nullable<Bool>,
        occupancy -> Nullable<Bool>,
        battery -> Nullable<Int4>,
        illuminance -> Nullable<Int4>,
        device_temperature -> Nullable<Float8>,
        power_outage_count -> Nullable<Int4>,
        timestamp -> Timestamp,
    }
}
