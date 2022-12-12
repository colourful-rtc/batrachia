mod base;
mod frame;
mod symbols;
mod abstracts;
mod media_stream;
mod media_stream_track;
mod observer;
mod rtc_datachannel;
mod rtc_icecandidate;
mod rtc_peerconnection;
mod rtc_peerconnection_configure;
mod rtc_session_description;
mod stream_ext;

pub use media_stream::MediaStream;
pub use media_stream_track::{
    MediaStreamTrack,
    MediaStreamTrackKind,
    VideoTrack,
    AudioTrack,
};

pub use stream_ext::{
    Sinker,
    SinkExt,
};

pub use rtc_icecandidate::RTCIceCandidate;
pub use rtc_peerconnection::RTCPeerConnection;
pub use frame::{
    AudioFrame,
    VideoFrame,
};

pub use rtc_datachannel::{
    DataChannelOptions,
    DataChannelPriority,
    DataChannelState,
    RTCDataChannel,
};

pub use rtc_peerconnection_configure::{
    BundlePolicy,
    IceTransportPolicy,
    RTCConfiguration,
    RTCIceServer,
    RtcpMuxPolicy,
};

pub use rtc_session_description::{
    RTCSessionDescription,
    RTCSessionDescriptionType,
};

pub use observer::{
    CreateDescriptionObserver,
    IceConnectionState,
    IceGatheringState,
    ObserverExt,
    Observer,
    ObserverPromisify,
    ObserverPromisifyExt,
    PeerConnectionState,
    SetDescriptionObserver,
    SignalingState,
};

/// By default, run() calls Thread::Current()->Run().
/// To receive and dispatch messages.
pub fn run() {
    RTCPeerConnection::run()
}
