-- Add salt column to users
alter table users add column salt text not null;