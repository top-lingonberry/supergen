# Supergen

Supergen is a very simple binary application which takes three values:
1. A username, in the form of a --username value in the command line.
2. A password, in the form of a --password value in the command line.
3. A database url, in the form of an environment variable by the name "SUPERGEN_DB_URL".

The application creates a table (if it doesn't exist) in the database to store a username (VARCHAR), salt (BYTEA) and pbkdf2 sha256 hashed password (BYTEA). This is sent to the database either securely (with a self generated SSL .pem cert) or without TLS.

This application was created with a single use purpose in mind. While I'm not opposed to developing it further, I personally have no need to do so at present.