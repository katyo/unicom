use std::sync::Arc;

#[cfg(feature = "tokio")]
use tokio_serial::{DataBits, FlowControl, Parity, SerialPortBuilder, SerialStream, StopBits};

use unicom::{Backend, BoxedConnect, BoxedConnection, BoxedConnector, Connector, Error, Url};

/// Serial port backend
///
/// Support connecting to devices via serial ports
#[derive(Clone)]
pub struct SerialPort {
    name: String,
    description: String,
}

impl Default for SerialPort {
    fn default() -> Self {
        Self {
            name: "serial-port".into(),
            description: "Support for serial port connections.".into(),
        }
    }
}

impl Backend for SerialPort {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn connector(&self, url: &Url) -> Option<BoxedConnector> {
        if (url.scheme() == "serial"
            || url.scheme() == "console"
            || url.scheme() == "tty"
            || url.scheme() == "com")
            && !url.has_host()
            && url.path() != "/"
        {
            let builder = tokio_serial::new(url.path(), 115200);
            let builder =
                if let Some(baud_rate) = url.fragment().and_then(|val| val.parse::<u32>().ok()) {
                    builder.baud_rate(baud_rate)
                } else {
                    builder
                };
            let builder = url
                .query_pairs()
                .fold(builder, |builder, (name, val)| match &*name {
                    "baud" | "baudrate" | "baud_rate" => {
                        if let Ok(baud_rate) = val.parse::<u32>() {
                            builder.baud_rate(baud_rate)
                        } else {
                            builder
                        }
                    }
                    "data" | "databits" | "data_bits" => match &*val {
                        "5" | "five" => builder.data_bits(DataBits::Five),
                        "6" | "six" => builder.data_bits(DataBits::Six),
                        "7" | "seven" => builder.data_bits(DataBits::Seven),
                        "8" | "eight" => builder.data_bits(DataBits::Eight),
                        _ => builder,
                    },
                    "flow" | "flowctrl" | "flowcontrol" | "flow_ctrl" | "flow_control" => {
                        match &*val {
                            "none" | "no" | "off" => builder.flow_control(FlowControl::None),
                            "soft" | "sw" | "software" => {
                                builder.flow_control(FlowControl::Software)
                            }
                            "hard" | "hw" | "hardware" => {
                                builder.flow_control(FlowControl::Hardware)
                            }
                            _ => builder,
                        }
                    }
                    "parity" => match &*val {
                        "none" | "no" => builder.parity(Parity::None),
                        "odd" | "od" => builder.parity(Parity::Odd),
                        "even" | "evn" | "ev" => builder.parity(Parity::Even),
                        _ => builder,
                    },
                    "stop" | "stopbits" | "stop_bits" => match &*val {
                        "1" | "one" => builder.stop_bits(StopBits::One),
                        "2" | "two" => builder.stop_bits(StopBits::Two),
                        _ => builder,
                    },
                    _ => builder,
                });
            let url = url.clone();
            Some(Arc::new(SerialConnector { url, builder }))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct SerialConnector {
    url: Url,
    builder: SerialPortBuilder,
}

impl Connector for SerialConnector {
    fn url(&self) -> &Url {
        &self.url
    }

    fn connect(&self) -> BoxedConnect {
        let this = self.clone();
        Box::pin(async move {
            let mut stm = SerialStream::open(&this.builder)
                .map_err(|err| Error::FailedConnect(err.to_string()))?;
            #[cfg(unix)]
            stm.set_exclusive(true)
                .map_err(|err| Error::FailedConnect(err.to_string()))?;
            Ok(Box::new(stm) as BoxedConnection)
        })
    }
}
