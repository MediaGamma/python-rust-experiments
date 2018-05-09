#[macro_use] extern crate cpython;

use cpython::{PyResult, Python, PyDict};

py_module_initializer!(hello_rust, inithello_rust, PyInit_hello_rust, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust."));
    try!(m.add(py, "foo", py_fn!(py, foo_py(a: PyDict))));
    Ok(())
});

fn foo_py(py: Python, dict_in: PyDict) -> PyResult<PyDict> {
    dict_in.items(py).iter().for_each(|item| {
        let (key, val) = item;
        println!("{} - {}", key, val);
    });

    dict_in.set_item(py, "foo", "bar")?;
    Ok(dict_in)
}
