create table tb_user (
    id bigserial primary key,
    first_name varchar(255) not null,
    last_name varchar(255),
    complete_name varchar(255),
    email varchar(255) not null,
    password varchar(255) not null,
    fcm_token varchar(255),
    active boolean not null default false,
    deleted boolean,
    created_at timestamptz not null,
    updated_at timestamptz,
    deleted_at timestamptz,
    constraint uq_u_email unique(email)
);