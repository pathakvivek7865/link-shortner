-- Add up migration script here
create table if not exists settings(
    id text default 'DEFAULT_SETTINGS' not null primary key,
    encrypted_global_api_key text not null
);

insert into settings (encrypted_global_api_key) values ('d1a88968b173df2ebf49a16a4aae1ace672265de6db9224d7631474995aac45d');