-- Create the komorebi database
CREATE DATABASE IF NOT EXISTS client 
  DEFAULT CHARACTER SET utf8mb4
  DEFAULT COLLATE utf8mb4_unicode_ci;

CREATE USER IF NOT EXISTS 'backend_user' @'%' IDENTIFIED BY 'backend_password';

GRANT ALL PRIVILEGES ON client.* TO 'backend_user' @'%';

FLUSH PRIVILEGES;
