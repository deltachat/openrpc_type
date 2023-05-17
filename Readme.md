# Openrpc type

A crate to parse (maybe later also serialize) OpenRPC documents.
OpenRPC is a specification for describing JsonRPC apis.

## Uses

We made this to use as a base for a code generator that generates wrapper code in multiple languages. [not realized yet, this crate is the first step]

See https://github.com/open-rpc/typings if you need a code generator for types right now.

## Places where this crate not follows the spec:

- `ServerObject.name` attribute is currently optional, where in the spec it is required, because the official examples don't follow the spec here, see: https://github.com/open-rpc/examples/issues/647

- `LinkObject.name` attribute is currently optional, same reason as above, see https://github.com/open-rpc/examples/issues/648

## Todo

- [ ] Make references work (`OrRef` type, currently it does not parse correctly)
- [ ] Implement automated testing (smoke tests are sufficient for now)
  - [x] add more examples (from openrpc)

### Ideas

- [ ] Maybe add some convenience function for resolving references? (only local ones remote ones will error for now I'd say)

### Less Priority

- [X] `ErrorObject.data` (maybe serde_json::Value?)
- [x] `LinkObject`
- [x] `ExampleObject`

## License

MIT or Apache, whatever suits you best.
Most Documentation comments are under Apache License 2.0, because they were taken from https://spec.open-rpc.org/.

## Testing Tips

run all tests, even if some fail:

```
cargo test --no-fail-fast
```
