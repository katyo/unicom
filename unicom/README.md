# Unified communication for IoT clients

The backends interface was introduced to support different connection methods. Each backend should provide the way to establish bidirectional connection for exchanging binary data with devices.

**IMPORTANT NOTE**: Async runtime feature should be selected explicitly.

## Supported features

* __futures__ Use [futures](https://docs.rs/futures/) only
* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)
