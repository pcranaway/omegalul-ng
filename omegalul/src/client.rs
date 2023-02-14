use crate::{
    api::responses::{EventsResponse, StartResponse, StatusResponse},
    randid::{self, RandID},
};

/// An Omegle client that stores relevant information about the session and allows for easier and,
/// most importantly, contextual calling of the API.
pub struct Client {
    pub randid: RandID,
    pub interests: Vec<String>,
    pub endpoint: String,
    pub language: String,
    pub cc: String,

    pub chat_id: Option<String>,

    rc: reqwest::blocking::Client,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            randid: randid::generate(),
            interests: vec![],
            endpoint: "https://omegle.com".to_string(),
            language: "en".to_string(),
            cc: "c7651d359c064edcb53d53f65e0265d9bf248fc3".to_string(),
            chat_id: None,
            rc: reqwest::blocking::ClientBuilder::default()
                .build()
                .expect("couldn't create client"),
        }
    }
}

impl Client {
    pub fn add_interest(mut self, value: String) {
        self.interests.push(value);
    }

    /// Blockingly starts a new chat.
    pub fn new_chat(&mut self) -> Result<StartResponse, anyhow::Error> {
        let request = self
            .rc
            .post(format!("{}/start", self.endpoint))
            .query(&("caps", "recaptcha2,t3"))
            .query(&("firstevents", "1"))
            .query(&("spid", ""))
            .query(&("randid", &self.randid))
            .query(&("cc", &self.cc)) // idk what this is
            .query(&("lang", &self.language))
            .build()?;

        let response = self.rc.execute(request)?.json::<StartResponse>()?;

        self.chat_id = Some(response.client_id.clone());

        Ok(response)
    }

    /// Blockingly requests the events of a chat.
    pub fn events(&self) -> Result<EventsResponse, anyhow::Error> {
        let request = self
            .rc
            .post(format!("{}/events", self.endpoint))
            .query(&("id", &self.chat_id))
            .build()?;

        let response = self.rc.execute(request)?.json::<EventsResponse>()?;

        Ok(response)
    }

    /// Blockingly sends a message to a chat.
    ///
    /// Returns whether disconnecting succeeded.
    pub fn send(&self, msg: String) -> Result<bool, anyhow::Error> {
        let request = self
            .rc
            .post(format!("{}/send", self.endpoint))
            .query(&("msg", msg))
            .query(&("id", &self.chat_id))
            .build()?;

        assert!(self.chat_id != None);

        let response = self.rc.execute(request)?.text()?;

        Ok(response == "win")
    }

    /// Blockingly disconnects from a chat.
    ///
    /// Returns whether disconnecting succeeded.
    pub fn disconnect(&self) -> Result<bool, anyhow::Error> {
        let request = self
            .rc
            .post(format!("{}/disconnect", self.endpoint))
            .query(&("id", &self.chat_id))
            .build()?;

        assert!(self.chat_id != None);

        let response = self.rc.execute(request)?.text()?;

        Ok(response == "win")
    }
}
/// Sends a request to Omegle's API on the /status path.
pub fn status() -> Result<StatusResponse, anyhow::Error> {
    let response = reqwest::blocking::get("https://omegle.com/status")?.json::<StatusResponse>()?;

    Ok(response)
}
