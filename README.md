# Supergen

Supergen is a very simple binary application which prompts for a username and password. Using an env variable by the name "SUPERGEN_DB_URL", it securely transfers the data to a Postgresql database.

The application creates a table (if it doesn't exist) in the database to store a username (VARCHAR), salt (BYTEA) and pbkdf2 sha256 hashed password (BYTEA). This is sent to the database securely with a self generated SSL .pem cert. The application assumes the certificate is stored in the same directory as the application, with the name "cert.pem".

This application was created with a single use purpose in mind. While I'm not opposed to developing it further, I personally have no need to do so at present.