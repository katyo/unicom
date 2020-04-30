# Raw TCP socket backend for unicom

This backend can be used to connect to devices which mapped to TCP ports, such as RS232-TO-TCP connectors.

**IMPORTANT NOTE**: Async runtime feature should be selected explicitly.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)
