extern crate amqp;

use amqp::{
    Session,
    Channel,
    Table,
    Basic,
    Options,
};

extern crate log;
extern crate simple_logger;

use log::Level;

use amqp::protocol::basic::{
    BasicProperties,
    Deliver,
};

use std::{
    str,
    time,
    thread,
};

use std::default::Default;

use crate::rabbitmq::types;
use crate::system::constants;

pub struct SessionRabbitmq
{
    pub _durable: bool,
    pub _session: Session,
    pub _channel: Channel,
    pub _prefetch_count: u16,
    pub _init: bool,
}

fn GetSession() -> Session
{
    let session = match Session::new(Options { .. Default::default() }){
         Ok(session) => session,
         Err(error) => panic!("Failed openning an amqp session: {:?}", error)
    };
    return session;
}

fn GetChannel(mut session:Session) -> Channel
{
    let channel = session.open_channel(1).ok().expect("Can not open a channel");
    return channel;
}


impl Default for SessionRabbitmq
{
    fn default() -> SessionRabbitmq
    {
        let session:Session = GetSession();
        let channel:Channel = GetChannel(session);
        let session_new:Session = GetSession();
        SessionRabbitmq
        {
            _durable: false,
            _session: session_new,
            _channel: channel,
            _prefetch_count: 1,
            _init: false,
        }
    }
}

impl SessionRabbitmq
{

    /// Refactor of the queue creation process.
    ///
    /// Args:
    ///
    /// `queue_name` - the name of the queue to declare
    fn declare_queue(&mut self, queue_name:&str)
    {
        warn!("Declaring queue for consumption");
        self._channel.queue_declare(
            queue_name,
            false,
            self._durable,
            false,
            false,
            false,
            Table::new()
        ).unwrap();
    }

    pub fn Create_session_and_channel(&mut self)
    {
        if self._init
        {
            debug!("Initialised Rabbitmq Connection = {}", constants::COMPONENT_NAME);
        }
        else
        {
            warn!("Creating session and channel");
            self._session = Session::open_url(types::QUEUE_URL).unwrap();
            self._channel = self._session.open_channel(1).unwrap();

            if self._prefetch_count != 0
            {
                self._channel.basic_prefetch(self._prefetch_count).unwrap();
            }
            self._init = true;
        }
    }

    fn terminate_session_and_channel(&mut self)
    {
        const CLOSE_REPLY_CODE: u16 = 200;
        const CLOSE_REPLY_TEXT: &str = "closing producer";
        self._channel.close(
            CLOSE_REPLY_CODE,
            CLOSE_REPLY_TEXT,
        ).unwrap();
        self._session.close(
            CLOSE_REPLY_CODE,
            CLOSE_REPLY_TEXT,
        );
    }

    pub fn publish(&mut self,topic: &str, message: &str)
    {

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
        ).unwrap();
    }

    pub fn Consume(&mut self)
    {
        let queue_name: &str = "";
        warn!("Beginning consumption");
        self.declare_queue(&queue_name);

        self._channel.exchange_declare(
            "topics",
            "topic",
            true,
            false,
            false,
            false,
            false,
            Table::new(),
        ).unwrap();

        self._channel.queue_bind(
            queue_name,
            types::EXCHANGE_NAME,
            types::POWER_NOTICE,
            false,
            Table::new(),
        ).unwrap();

        let mut expected:bool = true;

        while expected
        {
            for get_result in self._channel.basic_get(queue_name, false)
            {
                warn!("Received: {:?}", String::from_utf8_lossy(&get_result.body));
                get_result.ack();
                if(String::from_utf8_lossy(&get_result.body) == "50")
                {
                    expected = false;
                }
            }
        }

        warn!("[{} Consumer ] Started.", queue_name);
    }
}
