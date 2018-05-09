# hello_rust

## Setup

Install rust nighlyt using rustup

```
rustup install nightly
rustup default nightly
```

## Building

```
python setup.py develop
```

## Quick Check

```
python -c 'import hello_rust; print(hello_rust.foo({"hey": "there"}))'
```