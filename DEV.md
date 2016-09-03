Development setup
=================

## Base requirements

```sh
git
sqlite3
```

// TODO: other dependencies?

## Javascript

```sh
$ node --version
v6.2.1
```

```sh
cd ./client_src
npm install
webpack
```

NOTE: npm install webpack -g

### Generate static react components

This generates string (server-side) versions of *some* react components, and places them on /src/react_components.
These files are injected and inlined into the rust application.

```sh
build.sh
```

## Rust

```sh
$ rustc --version
rustc 1.13.0-nightly (497d67d70 2016-09-01)
```

1. install rust via https://www.rustup.rs/
2. ./buildrun.sh


## If build fails

NOTE: use nightly rust. see: https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust

```sh
rustup install nightly
```

```sh
rustup update
cargo update
```


On linux
========

NOTE: work in progress

## ubuntu provision

apt-get install git
apt-get install build-essential
apt-get install libssl-dev
apt-get install sqlite3 libsqlite3-dev

curl https://sh.rustup.rs -sSf | sh
rustup default nightly

git clone git@github.com:dashed/grokdb2.git
./buildrun.sh

