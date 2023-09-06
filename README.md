# Python plugin

[Python](https://www.python.org/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Caveats

This plugin only supports pre-builts via [indygreg/python-build-standalone](https://github.com/indygreg/python-build-standalone), and primarily only Python 3.

Building from source (with `python-build`), and supporting Python 2, will be supported in the future.

## Contributing

Build the plugin:

```shell
carpython build --target wasm32-wasi
```

Test the plugin by running `proto` commands. Requires proto >= v0.12.

```shell
proto install python-test
proto list-remote python-test
```
