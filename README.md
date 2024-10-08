[![codemp](https://code.mp/static/banner.png)](https://code.mp)

[![Actions Status](https://github.com/hexedtech/codemp-proto/actions/workflows/ci.yml/badge.svg)](https://github.com/hexedtech/codemp-proto/actions)
[![Crates.io Version](https://img.shields.io/crates/v/codemp-proto)](https://crates.io/crates/codemp-proto)
[![docs.rs](https://img.shields.io/docsrs/codemp-proto)](https://docs.rs/codemp-proto)
[![Crates.io License](https://img.shields.io/crates/l/codemp-proto)](https://github.com/hexedtech/codemp-proto/blob/dev/LICENSE)
[![Gitter](https://img.shields.io/gitter/room/hexedtech/codemp-proto)](https://gitter.im/hexedtech/codemp-proto)

> `codemp` is a **collaborative** text editing solution to work remotely.

It seamlessly integrates in your editor providing remote cursors and instant text synchronization,
as well as a remote virtual workspace for you and your team.

# Protocol
gRPC protocol specification for [`codemp`](https://github.com/hexedtech/codemp).

This is a [`prost`](https://github.com/tokio-rs/prost) crate: it will generated Rust code based on the protobuf defintions contained in the `proto/` directory.

From protobuf definition, `prost` will compile structs for all protocol messages but also server and client service implementations.

## Building
To compile this crate, `protoc` must be installed: [`prost` requires it to compile the protocol definition](https://docs.rs/prost/latest/prost/#protoc).
