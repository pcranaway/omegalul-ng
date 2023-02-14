use serde::{Serialize, Deserialize};

/// Maps the response that the path /status on the omegle endpoint returns.
///
/// Contains different stats and useful informatino (such as the available endpoitns) about Omegle
/// at the moment.
///
/// This is an example response:
/// ```json
/// {
///     "count": 41709,
///     "antinudeservers": [
///         "waw4.omegle.com", "waw1.omegle.com",
///         "waw2.omegle.com", "waw3.omegle.com"
///     ],
///     "spyQueueTime": 112.19500000476837,
///     "rtmfp": "rtmfp://p2p.rtmfp.net",
///     "antinudepercent": 1.0,
///     "spyeeQueueTime": 261.56370000839235,
///     "timestamp": 1676390007.480977,
///     "servers": [
///         "front25", "front43", "front21", "front20",
///         "front26", "front23", "front5", "front39",
///         "front27", "front19", "front40", "front36",
///         "front48", "front32", "front37", "front45",
///         "front13", "front4", "front22", "front7",
///         "front29", "front41", "front30", "front38",
///         "front18", "front34", "front12", "front3",
///         "front11", "front35", "front9", "front15",
///         "front44", "front17", "front31", "front46",
///         "front1", "front8", "front28", "front47",
///         "front16", "front42", "front10", "front6",
///         "front14", "front2", "front33", "front24"
///     ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    /// Amount of online people.
    pub count: i32,

    /// ?
    #[serde(rename = "antinudeservers")]
    pub anti_nude_servers: Vec<String>,

    /// ?
    #[serde(rename = "spyQueueTime")]
    pub spy_queue_time: f32,

    /// ?
    pub rtmfp: String,

    /// ?
    #[serde(rename = "spyQueueTime")]
    pub anti_nude_percent: f32,

    /// ?
    #[serde(rename = "spyeeQueueTime")]
    pub spyee_queue_time: f32,

    /// Current timestamp.
    pub timestamp: f32,

    /// Names of the available endpoints.
    ///
    /// Names as in the subdomain of the domain omegle.com
    pub servers: Vec<String>
}

/// Maps the response that the path /start (with the required query string) on the omegle endpoint
/// returns.
#[derive(Serialize, Deserialize, Debug)]
pub struct StartResponse {
    #[serde(rename = "clientID")]
    pub client_id: String,

    /// Events. Each vec contains and event and its arguments.
    pub events: Vec<Vec<String>>
}

// Response that is returned when a request is sent on Omegle's API on the /events path with the
// required query string.
pub type EventsResponse = Vec<Vec<String>>;
