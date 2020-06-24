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
3. set cron to check every day