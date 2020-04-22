/*!

# Raw serial port backend for unicom

This backend can be used for interfacing with devices connected via serial ports.

**IMPORTANT NOTE**: Async framework feature should be selected, because there is no default.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* [TODO] __async-std__ Use [async-std](https://docs.rs/async-std/)

*/

use std::{
    path::PathBuf,
    sync::Arc,
};

#[cfg(feature = "tokio")]
use tokio_serial::{DataBits, FlowControl, Parity, Serial, SerialPortSettings, StopBits};

use unicom::{
    Url, Error,
    Backend, Connector, BoxedConnector, BoxedConnect, BoxedConnection,
};

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
        if (url.scheme() == "serial" || url.scheme() == "tty")
            && !url.has_host()
            && url.path() != "/"
        {
            let path = url.path().into();
            let mut settings = url.query_pairs().fold(
                SerialPortSettings::default(),
                |mut settings, (name, val)| {
                    match &*name {
                        "baud" | "baudrate" | "baud_rate" => {
                            if let Ok(baud_rate) = val.parse::<u32>() {
                                settings.baud_rate = baud_rate;
                            }
                        }
                        "data" | "databits" | "data_bits" => match &*val {
                            "5" | "five" => settings.data_bits = DataBits::Five,
                            "6" | "six" => settings.data_bits = DataBits::Six,
                            "7" | "seven" => settings.data_bits = DataBits::Seven,
                            "8" | "eight" => settings.data_bits = DataBits::Eight,
                            _ => (),
                        },
                        "flow" | "flowctrl" | "flowcontrol" | "flow_ctrl" | "flow_control" => {
                            match &*val {
                                "none" | "no" | "off" => settings.flow_control = FlowControl::None,
                                "soft" | "sw" | "software" => {
                                    settings.flow_control = FlowControl::Software
                                }
                                "hard" | "hw" | "hardware" => {
                                    settings.flow_control = FlowControl::Hardware
                                }
                                _ => (),
                            }
                        }
                        "parity" => match &*val {
                            "none" | "no" => settings.parity = Parity::None,
                            "odd" | "od" => settings.parity = Parity::Odd,
                            "even" | "evn" | "ev" => settings.parity = Parity::Even,
                            _ => (),
                        },
                        "stop" | "stopbits" | "stop_bits" => match &*val {
                            "1" | "one" => settings.stop_bits = StopBits::One,
                            "2" | "two" => settings.stop_bits = StopBits::Two,
                            _ => (),
                        },
                        _ => (),
                    }
                    settings
                },
            );
            if let Some(val) = url.fragment() {
                if let Ok(baud_rate) = val.parse::<u32>() {
                    settings.baud_rate = baud_rate;
                }
            }
            let url = url.clone();
            Some(Arc::new(SerialConnector { url, path, settings }))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct SerialConnector {
    url: Url,
    path: PathBuf,
    settings: SerialPortSettings,
}

impl Connector for SerialConnector {
    fn url(&self) -> &Url {
        &self.url
    }

    fn connect(&self) -> BoxedConnect {
        let this = self.clone();
        Box::pin(async move {
            let stm = Serial::from_path(&this.path, &this.settings)
                .map_err(|e| Error::FailedConnect(e.to_string()))?;
            Ok(Box::new(stm) as BoxedConnection)
        })
    }
}
