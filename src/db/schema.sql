CREATE DATABASE graduation;


CREATE TABLE event_attendance (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    will_attend BOOLEAN NOT NULL
);
