# Python plugin

[Python](https://www.python.org/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Caveats

This will install a pre-built version from [indygreg/python-build-standalone](https://github.com/indygreg/python-build-standalone), which doesn't support all versions, only Python 3.

Building from source directly (with `python-build`), and supporting Python 2, will be fully supported in the future.

### Global packages

When globals are installed with `proto install-global python`, we install them using `pip --user`, which installs them to `~/.local/lib/pythonX.Y/site-packages` or `~/.local/bin` on Linux and macOS, and `~/AppData/Roaming/Python/PythonXY/Scripts` on Windows.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands. Requires proto >= v0.17.

```shell
proto install python-test
proto list-remote python-test
```
