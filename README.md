<p align="center"><a href="https://github.com/hexedtech/codemp"><img alt="codemp logo" src="https://codemp.dev/static/codemp-banner.png" height="100"/></a></p>

# Protocol
gRPC protocol specification for [codemp](https://github.com/hexedtech/codemp).

This is a [prost](https://github.com/tokio-rs/prost) crate: it will generated Rust code based on the protobuf defintions contained in the `proto/` directory.

From protobuf definition, `prost` will compile structs for all protocol messages but also server and client service implementations.

## Building
To compile this crate, `protoc` must be installed: [`prost` requires it to compile the protocol definition](https://docs.rs/prost/latest/prost/#protoc).
