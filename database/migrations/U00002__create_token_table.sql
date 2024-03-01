create table tb_token (
    id bigserial primary key,
    token varchar(100) not null,
    token_type varchar(50) not null,
    is_validated boolean not null,
    created_at timestamptz not null,
    validated_at timestamptz,
    expire_at timestamptz,
    user_id bigint not null,
    constraint uq_t_token_user_id unique(token, user_id),
    constraint fk_t_user_id foreign key (user_id) references tb_user(id)
);