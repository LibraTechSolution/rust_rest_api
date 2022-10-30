-- Your SQL goes here
CREATE TABLE "phone_report"
(
    id SERIAL PRIMARY KEY,
    phone_number VARCHAR NOT NULL,
    origin_name VARCHAR NOT NULL,
    count_confirm INT NOT NULL,
    count_report INT NOT NULL,
    date_created VARCHAR NOT NULL,
    date_modified VARCHAR NOT NULL


)
