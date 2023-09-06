# Python plugin

[Python](https://www.python.org/) WASM plugin for [proto](https://github.com/moonrepo/proto).

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
