# Unified communication for IoT clients

The backends interface was introduced to support different connection methods.

Each backend should provide the way to establish bidirectional connection for exchange binary data with devices.

**IMPORTANT NOTE**: Async feature should be selected by consumer.

## Supported features

* __futures__ Use [futures](https://docs.rs/futures/) only
* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)

## Built-in backends

* _unicom-tcp_ TCP socket connection
* _unicom-unix_ Unix-domain socket connection
* _unicom-serial_ Serial port connection
