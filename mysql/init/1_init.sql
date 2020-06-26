CREATE DATABASE holiday_bot;

CREATE USER 'rocket'@'%' IDENTIFIED BY 'password';

GRANT ALL PRIVILEGES ON holiday_bot.* TO rocket;

-- FLUSH PRIVILEGES;

