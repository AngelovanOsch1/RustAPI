CREATE TABLE login_credentials (
    user_id INT IDENTITY(1,1) PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE user_information (
    info_id INT IDENTITY(1,1) PRIMARY KEY,
    user_id INT,
    email VARCHAR(255) NOT NULL UNIQUE,
    FOREIGN KEY (user_id) REFERENCES login_credentials(user_id)
);