# Raw TCP socket backend for unicom

This backend can be used to connect to devices which mapped to TCP ports, such as RS232-TO-TCP connectors.

**IMPORTANT NOTE**: Async crate feature should be selected, because no default is set.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)
