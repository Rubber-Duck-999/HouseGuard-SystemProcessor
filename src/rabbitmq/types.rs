pub const QUEUE_URL: &str = "amqp://guest:guest@localhost:5672/";
pub const EXCHANGE_NAME: &str = "topics";
pub const REQUEST_POWER: &str = "Request.Power";
pub const ISSUE_NOTICE: &str = "Issue.Notice";
pub const EVENT_SYP: &str = "Event.SYP";
pub const FAILURE_COMPONENT: &str = "Failure.Component";

pub const START_UP_FAILURE_SEVERITY: u16 = 3;
pub const RABBITMQ_SEVERITY: u16 = 6;

#[derive(Serialize, Deserialize, Debug)]
pub struct request_power 
{
    pub _power: String,
    pub _severity: u16,
    pub _component: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct issue_notice
{
    pub _severity: u16,
    pub _component: String,
    pub _action: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct event_syp
{
    pub _severity: u16,
    pub _error: String,
    pub _time: String,
    pub _component: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct failure_component
{
    pub _time: String,
    pub _type: u16,
    pub _severity: String,
}