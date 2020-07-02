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

### using rls
* if rls is down with message: "thread '<unnamed>' panicked at 'unexpected error reading save-analysis directory:  
 Os { code: 22, kind: InvalidInput, message: "Invalid argument" }'", maybe fixable use "docker-compose reload" command
* many times panic rls, use wsl and docker at the same times like below
1. enable wsl
2. git clone this in wsl
3. create symbolic link form here at workspace (maybe required)
4. create docker container
5. edit source by remote wsl access
6. when change source code, recompile in docker container  
(in wsl1, compile will be failure because install diesel-cli will be failure )
* error occurred in Cargo.toml with "could not compile rocket_codegen", but could compile.  
  temporary comment out all of dependencies, and then follow rls instructions and put it back.


### trouble shooting


* when cargo run, but failed with following messages
```
Error: Database configuration failure: 'holiday_bot'  
    => Error: A table named `databases` was not found for this configuration  
Error: Rocket failed to launch due to failing fairings:  
```
It occurs when no [global.databases] section in Rocket.toml  

