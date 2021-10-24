# Raw Unix-domain socket backend for unicom

This backend can be used to connect to device emulators which mapped to Unix-domain socket.

**IMPORTANT NOTE**: Async runtime feature should be selected explicitly.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)
