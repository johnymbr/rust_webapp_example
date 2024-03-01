create table tb_refresh_token (
    id bigserial primary key,
    uuid varchar(255) not null,
    is_revoked boolean not null default false,
    created_at timestamptz not null,
    revoked_at timestamptz,
    user_id bigint not null,
    constraint uq_rt_uuid_user_id unique(uuid, user_id),
    constraint fk_rt_user_id foreign key (user_id) references tb_user(id)
);