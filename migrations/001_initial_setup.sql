-- migrations/001_initial_setup.sql

CREATE TABLE userss (
    id INT PRIMARY KEY IDENTITY,
    username NVARCHAR(255) NOT NULL,
    password_hash NVARCHAR(255) NOT NULL
);
