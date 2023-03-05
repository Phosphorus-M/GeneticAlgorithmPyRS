import sys
from setuptools import find_packages, setup

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call(
        [sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import Binding, RustExtension

setup_requires = ['setuptools-rust>=1.5.2']
install_requires = []

setup(
    name='genetic_algorithm_py_rs',
    version='0.1',
    classifiers=[
        'License :: OSI Approved :: MIT License',
        'Development Status :: 1 - Planning',
        'Intended Audience :: Developers',
        'Programming Language :: Python',
        'Programming Language :: Rust',
        'Operating System :: POSIX',
        'Operating System :: MacOS :: MacOS X',
    ],
    rust_extensions=[RustExtension('genetic_algorithm_py_rs._genetic_algorithm', 'Cargo.toml', binding=Binding.PyO3)],
    packages=find_packages(),
    zip_safe=False
)