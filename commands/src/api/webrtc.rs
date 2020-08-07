use serde::{Deserialize, Serialize};

/// WebRTC client -> server requests
#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    /// Get offer from streamer
    GetOffer(GetOfferRequest),
    /// Set offer answer from client
    SetAnswer(SetAnswerRequest),
    /// Add client's ICE candidate
    AddCandidate(AddCandidateRequest),
    /// Get remote ICE candidates from streamer
    GetCandidates(GetCandidatesRequest),
    /// Force hang up (close client's streaming session)
    HangUp(HangUpRequest),
}

/// Request offer form the streamer
#[derive(Debug, Serialize, Deserialize)]
pub struct GetOfferRequest {
    /// Stream UUID
    pub stream_id: String,
}

/// ICE server entry
///
/// Refer to https://developer.mozilla.org/en-US/docs/Web/API/RTCIceServer
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IceServer {
    /// ICE servers URL's array. Note that payload is not an URI, it's format is like "stun:stun.l.google.com:19302"  
    pub urls: Vec<String>,
    /// Credential (TURN only)
    pub credential: Option<String>,
    /// Username (TURN only)
    pub username: Option<String>,
}

/// Response on GetOfferRequest
#[derive(Serialize, Deserialize, Debug)]
pub struct GetOfferResponse {
    /// Peer id identifier, should be user in all future calls
    peer_id: String,
    /// ICE servers settings for peer connection construction
    #[serde(rename = "iceServers")]
    ice_servers: Vec<IceServer>,
    /// SDP payload
    pub sdp: String,
    /// SDP type
    pub r#type: String,
}

/// Set offer answer request from client
#[derive(Serialize, Deserialize, Debug)]
pub struct SetAnswerRequest {
    /// SDP payload
    pub sdp: String,
    /// SDP type
    pub r#type: String,
    /// peer id (received from URL)
    pub peer_id: Option<String>,
}

/// Boolean response on SDP answer request
#[derive(Serialize, Deserialize, Debug)]
pub struct SetAnswerResponse(bool);

/// ICE candidate.
///
/// Refer to https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IceCandidate {
    candidate: String,
    sdp_m_line_index: u32,
    sdp_mid: String,
}

/// Set ICE candidate request
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCandidateRequest {
    /// ICE candidate
    pub candidate: IceCandidate,
    /// peer id (received from URL)
    pub peer_id: Option<String>,
}

/// Boolean response on set candidate request
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCandidateResponse(bool);

/// Get remote candidates list request
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCandidatesRequest {
    /// peer id (received from URL)
    pub peer_id: String,
}

/// Response on get ICE candidates request
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCandidatesResponse {
    /// Is the list complete flag
    /// Request MUST make another GetCandidatesRequest request till "completed" flag isn't true
    /// (poll for remote candidates)
    completed: bool,
    /// Remote ICE candidates list, every entry must be appended to the client's peer connection
    candidates: Vec<IceCandidate>,
}

/// Hang up request
#[derive(Serialize, Deserialize, Debug)]
pub struct HangUpRequest {
    /// peer id (received from URL)
    pub peer_id: String,
}

/// Hang up response
#[derive(Serialize, Deserialize, Debug)]
pub struct HangUpResponse(bool);

/// Responses from streamer
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    GetOffer(GetOfferResponse),
    SetAnswer(SetAnswerResponse),
    AddCandidate(AddCandidateResponse),
    GetCandidates(GetCandidatesResponse),
    HangUp(HangUpResponse),
}
