# Python plugin

[Python](https://www.python.org/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Caveats

If `python-build` exists on the host machine, this will be used to install python. Otherwise, a pre-built version will be downloaded from [indygreg/python-build-standalone](https://github.com/indygreg/python-build-standalone), which doesn't support all versions, only Python 3.

Building from source directly (with `python-build`), and supporting Python 2, will be fully supported in the future.

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
