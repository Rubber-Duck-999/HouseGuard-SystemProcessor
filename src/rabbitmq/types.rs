pub const QUEUE_URL: &str = "amqp://guest:guest@localhost:5672/";
pub const EXCHANGE_NAME: &str = "topics";
pub const REQUEST_POWER: &str = "Status.Update";
pub const EVENT_SYP: &str = "Event.SYP";
pub const STATUS_SYP: &str = "Status.SYP";
pub const FAILURE_COMPONENT: &str = "Failure.Component";

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusUpdate {
    pub cpu_usage: u16,
    pub memory_total: u16,
    pub memory_used: u16,
    pub images: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventSyp {
    pub message: String,
    pub time: String,
    pub component: String,
    pub event_type_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailureComponent {
    pub time: String,
    pub type_of_failure: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusSYP {
    pub temperature: f32,
    pub memory_left: u64,
    pub highest_usage: f32,
}
