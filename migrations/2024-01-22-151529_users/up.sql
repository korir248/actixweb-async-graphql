-- Your SQL goes here
create table if not exists users(
    id serial primary key,
    username varchar,
    email varchar not null unique,
    password varchar not null
);