# hello_rust

A simple example of using rust project to build Python modules.
Current example create a module called `hello_rust` with a single fuction named
`foo`.

`foo` accepts a python dict, iterates over and prints the key/value pairs, and
lastly augments the dict with a new "foo"/"bar" key/value pair.

## Setup

Install rust nighlyt using rustup

```
rustup install nightly
rustup default nightly
```

## Building

```
$ python setup.py develop
```

## Quick Check

```
python -c 'import hello_rust; print(hello_rust.foo({"hey": "there"}))'
```
Expected output
```
hey - there
{'hey': 'there', 'foo': 'bar'}
```


## References

- [Setup Tools Rust](https://github.com/PyO3/setuptools-rust)
- [Rust CPython](https://github.com/dgrunwald/rust-cpython)
- [CPython Crate docs](http://dgrunwald.github.io/rust-cpython/doc/cpython/index.html)
- [Rust CPython walkthrough](https://dvigneshwer.github.io/posts/2016/04/Rust-Python/)