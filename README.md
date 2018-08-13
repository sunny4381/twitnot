# How to build

Debug build:

~~~sh
$ cargo clean
$ cargo build
   Compiling num-traits v0.1.37
   Compiling semver v0.1.20
...

    Finished dev [unoptimized + debuginfo] target(s) in 1m 02s
~~~

Release build:

~~~sh
$ cargo clean
$ cargo build --release
   Compiling core-foundation-sys v0.2.3
   Compiling winapi-build v0.1.1
...
   Finished release [optimized] target(s) in 2m 06s
~~~

# How to use

Run via cargo command:

~~~
$ cargo run -- --help                                                            (git)-[master]
    Finished dev [unopt   imized + debuginfo] target(s) in 0.13s
     Running `target/debug/twitnot --help`
Tweet Monitor & Notification.

Usage:
  twitnot init <consumer-key> [--secret=<consumer-secret>] [--db=<database-file>] [--gmail-username=<gmail-username>] [--gmail-password=<gmail-password>]
  twitnot add <screen-name>
  twitnot list [<screen-name>] [--max=<max-count>]
  twitnot remove <screen-name>
  twitnot check-updates [--screen-name=<screen-name>]
  twitnot (-h | --help)
Options:
  -h, --help     Show this screen.
  --secret=<consumer-secret> Specify consucmer secret.
  --screen-name=<screen-name> Specify screen name.
  --db=<databaase-file> Specify database file.
  --max=<max-count> Specify max count of tweet [default is 10].
~~~

Run native binary directly:

~~~
$ ./target/release/twitnot --help
Tweet Monitor & Notification.

Usage:
  twitnot init <consumer-key> [--secret=<consumer-secret>] [--db=<database-file>] [--gmail-username=<gmail-username>] [--gmail-password=<gmail-password>]
  twitnot add <screen-name>
  twitnot list [<screen-name>] [--max=<max-count>]
  twitnot remove <screen-name>
  twitnot check-updates [--screen-name=<screen-name>]
  twitnot (-h | --help)
Options:
  -h, --help     Show this screen.
  --secret=<consumer-secret> Specify consucmer secret.
  --screen-name=<screen-name> Specify screen name.
  --db=<databaase-file> Specify database file.
  --max=<max-count> Specify max count of tweet [default is 10].
~~~


# How to update dependant packages



# How to update rust

before we update rust:

~~~sh
$ rustc --version
rustc 1.23.0 (766bd11c8 2018-01-01)
$ rustup --version
rustup 1.13.0 (ea9259c1b 2018-07-16)
~~~

do update rust:

~~~sh
$ rustup update
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2018-08-02, rust version 1.28.0 (9634041f0 2018-07-30)
...
info: installing component 'rust-docs'
info: checking for self-updates

   stable-x86_64-apple-darwin updated - rustc 1.28.0 (9634041f0 2018-07-30)
  nightly-x86_64-apple-darwin updated - rustc 1.30.0-nightly (73c78734b 2018-08-05)

~~~

after we update rust:

~~~sh
$ rustc --version
rustc 1.28.0 (9634041f0 2018-07-30)
$ rustup --version
rustup 1.13.0 (ea9259c1b 2018-07-16)
~~~
