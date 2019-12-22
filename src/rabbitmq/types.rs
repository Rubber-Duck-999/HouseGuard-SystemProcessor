pub const QUEUE_URL: &str = "amqp://guest:guest@localhost:5672/";
pub const EXCHANGE_NAME: &str = "topics";
pub const REQUEST_POWER: &str = "Request.Power";
pub const ISSUE_NOTICE: &str = "Issue.Notice";
pub const EVENT_SYP: &str = "Event.SYP";
pub const FAILURE_COMPONENT: &str = "Failure.Component";

pub const START_UP_FAILURE_SEVERITY: u16 = 3;
pub const RABBITMQ_SEVERITY: u16 = 6;
pub const SHUTDOWN: &str = "shutdown";
pub const RESTART: &str = "restart";
pub const RESTART_SET: bool = true;
pub const SHUTDOWN_SET: bool = false;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPower 
{
    pub power: String,
    pub severity: u16,
    pub component: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueNotice
{
    pub severity: u16,
    pub component: String,
    pub action: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventSyp
{
    pub severity: u16,
    pub error: String,
    pub time: String,
    pub component: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailureComponent
{
    pub time: String,
    pub type_of_failure: u16,
    pub severity: String,
}