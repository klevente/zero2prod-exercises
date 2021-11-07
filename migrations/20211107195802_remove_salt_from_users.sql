-- remove unused row salt from users
alter table users drop column salt;
