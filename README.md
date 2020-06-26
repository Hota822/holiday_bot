# What

This is a line bot to notification before public holiday

# Requirement and built environment

### Requirment
* follow docker
* I built this under Windows Hyper-V. So, environments under Windows docker toolbox, Linux and Mac are not tested

### Built environment
* rust, cargo nightly 1.46.0
* Nginx ? now implementing
* Mysql 8.0

# How to use

1. prepare environment
2. modify code/holiday_bot/.env_example and rename to .env.
3. migration database (detail operation is below)
4. set cron to check every day

### Migration
1. install diesel_cli(maybe installed)

```bash
cargo install diesel_cli --no-default-features --features mysql
```

2. create database and user, add permission
code is in mysql/init/1_init.sql.
execute command in mysql container and after logged in.



# Others
* if rls is down with message: "thread '<unnamed>' panicked at 'unexpected error reading save-analysis directory: Os { code: 22, kind: InvalidInput, message: "Invalid argument" }'", maybe fixable use "docker-compose reload" command

