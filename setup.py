from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='hello-rust',
    version='1.0',
    rust_extensions=[
        RustExtension(
            'hello_rust',
            'Cargo.toml',
            binding=Binding.RustCPython,
        )
    ],
    packages=['hello_rust'],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False)
