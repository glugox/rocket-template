{{Project-name}}
==================

[![Build Status](https://travis-ci.com/glugox/{{project-name}}.svg?branch=master)](https://travis-ci.com/glugox/{{project-name}})
[![codecov](https://codecov.io/gh/glugox/{{project-name}}/branch/master/graph/badge.svg)](https://codecov.io/gh/glugox/{{project-name}})

{{Project-name}} service for Glugate as part of microservice architecture.

Prerequirements:
-------------
* Diesel cli `cargo install diesel_cli` (Needed initializing database and for running tests)


Dependencies:
-------------
* `rocket` Web framework
* `diesel` ORM and Query Builder
* `serde` Serialization framework
* `validator` Validation library
* `log` and `stderrlog` Logging facade
* `clap` Library for parsing command line arguments and subcommands
* `slug` Generating slugs from unicode strings
* `rust-crypto` Cryptographic algorithms used for auth password management
* `rand` Random number generation
* `chrono` Date and time library
* `dotenv` Loading from local .env file
* `jsonwebtoken` Authentication
* `assert_cli` Integration testing
* `run_script` Used only in tests for generating test database to distinguish the main database when running tests


### Getting started

Install [nightly](https://www.rust-lang.org/en-US/install.html)
```sh
# install rustup
curl https://sh.rustup.rs -sSf | sh

rustup install nightly

# start postgresql and seed the database
cargo install diesel_cli --no-default-features --features "postgres"
diesel setup

cargo run
```

### Testing
Simply run:
```sh
cargo test -- --test-threads 1
```
This will try to create *_test database and run all migrations, so make sure you have correct postgress access in `./env.test` file. 
For development, you probably want to set the username to `postgres` and password to corresponding (main) postgres password.

You can also check Postman testing. See `/tests/Glugate.postman_collection.json` flie which can be imported into Postman.
Note that Postman test are running against the main database ( not the *_test one as integrations test do ), 
but on successful tests, all the test data should be cleaned up as we call DELETE method at the end.
TODO: Looks like the main database must be clear from test {{database-name}}. So currently , in order to pass all tests, it is best to run them against empty database.

To run restclient file tests you can run ad hoc testing. See `/tests/dev.http`

Note that in order to run the Postman and http tests, you need to run the application first.

### How it works
`diesel` cli uses `.env` file.
Rocket sets database configuration from `.env` file.
Checkout Rocket's amazing [guide](https://rocket.rs/guide/)
