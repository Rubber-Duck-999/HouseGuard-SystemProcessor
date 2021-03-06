extern crate amqp;

use amqp::{Basic, Channel, Options, Session, Table};

extern crate log;
extern crate simple_logger;
extern crate serde_yaml;

use crate::rabbitmq::types;

use amqp::protocol::basic::BasicProperties;

use std::str;

use std::default::Default;

pub struct MessagePower {
    pub _component: String,
    pub _count: u16,
    pub _state: u16,
}

pub struct SessionRabbitmq {
    pub _durable: bool,
    pub _session: Session,
    pub _channel: Channel,
    pub _prefetch_count: u16,
    pub _init: bool,
}

fn get_passcode_file() -> Result<String, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("SYP.yml")?;
    let d: String = serde_yaml::from_reader(f)?;
    println!("Read YAML string");
    Ok(d)
}
fn get_session() -> Session {
    let pass: Result<String, Box<dyn std::error::Error>> = get_passcode_file();
    match pass {
        Ok(code) => {
            let session = match Session::new(Options {
                password: code,
                ..Default::default()
            }) {
                Ok(session) => session,
                Err(error) => panic!("Failed opening an amqp session: {:?}", error),
            };
            return session;
        }
        Err(_err) => {
            let session = match Session::new(Options {
                password: "N/A".to_string(),
                ..Default::default()
            }) {
                Ok(session) => session,
                Err(error) => panic!("Failed opening an amqp session: {:?}", error),
            };
            return session;
        }
    }

}

fn get_channel(mut session: Session) -> Channel {
    let channel = session
        .open_channel(1)
        .ok()
        .expect("Can not open a channel");
    return channel;
}

impl Default for SessionRabbitmq {
    fn default() -> SessionRabbitmq {
        let session: Session = get_session();
        let channel: Channel = get_channel(session);
        let session_new: Session = get_session();
        SessionRabbitmq {
            _durable: false,
            _session: session_new,
            _channel: channel,
            _prefetch_count: 1,
            _init: false,
        }
    }
}

impl SessionRabbitmq {
    /// Refactor of the queue creation process.
    ///
    /// Args:
    ///
    /// `queue_name` - the name of the queue to declare
    fn declare_queue(&mut self, queue_name: &str) {
        warn!("Declaring queue for consumption");
        self._channel
            .queue_declare(
                queue_name,
                false,
                self._durable,
                false,
                false,
                false,
                Table::new(),
            )
            .unwrap();
    }

/*     pub fn create_session_and_channel(&mut self) {
        if self._init {
            debug!(
                "Initialised Rabbitmq Connection = {}",
                constants::COMPONENT_NAME
            );
        } else {
            warn!("Creating session and channel");
            self._session = Session::open_url(types::QUEUE_URL).unwrap();
            self._channel = self._session.open_channel(1).unwrap();

            if self._prefetch_count != 0 {
                self._channel.basic_prefetch(self._prefetch_count).unwrap();
            }
            self._init = true;
        }
    } */
/* 
    fn terminate_session_and_channel(&mut self) {
        const CLOSE_REPLY_CODE: u16 = 200;
        const CLOSE_REPLY_TEXT: &str = "closing producer";
        self._channel
            .close(CLOSE_REPLY_CODE, CLOSE_REPLY_TEXT)
            .unwrap();
        self._session.close(CLOSE_REPLY_CODE, CLOSE_REPLY_TEXT);
    } */

    pub fn publish(&mut self, topic: &str, message: &str) {
        debug!("Publishing");
        self._channel.basic_publish(
                types::EXCHANGE_NAME,
                topic,
                false,
                false,
                BasicProperties {
                    content_type: Some("text".to_string()),
                    ..Default::default()
                },
                message.to_string().into_bytes(),
            )
            .ok().expect("Failed publishing");
        debug!("Published");
    }

    pub fn consume(&mut self) {
        warn!("Beginning consumption");
        self.declare_queue("");

        self._channel
            .exchange_declare(
                "topics",
                "topic",
                true,
                false,
                false,
                false,
                false,
                Table::new(),
            )
            .unwrap();

        self._channel
            .queue_bind(
                "",
                types::EXCHANGE_NAME,
                types::REQUEST_POWER,
                false,
                Table::new(),
            )
            .unwrap();

        warn!("[{} Consumer ] Created.", "");
    }

    
    pub fn consume_get(&mut self) -> MessagePower {
        let mut component = "";
        let mut count = 0;
        let mut state = 0;
        let mut message = types::RequestPower::default();
        for get_result in self._channel.basic_get("", false) {
            if get_result.reply.routing_key.contains(types::REQUEST_POWER) {
                warn!("Received {}", types::REQUEST_POWER);

                if String::from_utf8_lossy(&get_result.body).contains("component")
                {
                    message = serde_json::from_str(&String::from_utf8_lossy(&get_result.body)).unwrap();
                    component = &message.component;
                    warn!("Received a power request for {}", component);
                    state = message.state;
                    count = count + 1;
                }
            }
            get_result.ack();
        }
        let temp = MessagePower {
            _component: component.to_string(),
            _state: state,
            _count: count,
        };
        return temp;
    }
}
