-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE phone_report (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  count_report SERIAL NOT NULL,
  date_modified VARCHAR NOT NULL,
  date_created VARCHAR NOT NULL
)