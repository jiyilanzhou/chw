
drop table if exists item;
drop table if exists list;

create table list (
    id serial primary key,
    title varchar(150) not null
);

create table item (
    id serial  primary key,
    title varchar(150) not null,
    checked boolean not null default false,
    list_id integer not null,
    foreign key(list_id) references list(id)
);

insert into list(title) values('list 1'),('list 2');
insert into item(title,list_id)
    values('item 1',1),('item 2',1),('item 1',2);