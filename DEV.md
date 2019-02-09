Development setup
=================

## Base requirements

```sh
git
sqlite3
```

### OSX

```sh
brew install sqlite3

brew install openssl
# https://github.com/sfackler/rust-openssl/tree/b8fb29db5c246175a096260eacca38180cd77dd0#osx
# https://github.com/sfackler/rust-openssl/issues/255#issuecomment-257462451
```
// TODO: other dependencies? libc

## Assets

```sh
cd assets
unzip MathJax-2.6.1.zip
mv MathJax-2.6.1 mathjax
```

## Javascript

```sh
$ node --version
v6.2.1
```

```sh
cd ./client_src
yarn install
yarn webpack
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
rustc 1.23.0-nightly (2be4cc040 2017-11-01)
```

1. install rust via https://www.rustup.rs/
2. ./buildrun.sh


## If build fails

NOTE: use nightly rust. see: https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust

```sh
rustup override set nightly-2017-11-01
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

