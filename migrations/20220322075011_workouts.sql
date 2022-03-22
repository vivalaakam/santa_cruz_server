CREATE TABLE IF NOT EXISTS workouts
(
    id         integer
        constraint table_name_pk primary key autoincrement,
    status     integer  not null default (0),
    day        datetime not null default (datetime('now')),
    created_at datetime not null default (datetime('now')),
    updated_at datetime not null default (datetime('now'))
);

create unique index workouts_id_uindex
    on workouts (id);
