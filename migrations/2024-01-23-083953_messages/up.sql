-- Your SQL goes here
create table if not exists messages(
    id serial primary key,
    sender integer references users(id) not null,
    receiver integer references users(id) not null,
    text varchar not null
);