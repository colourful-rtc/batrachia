use super::base::*;
use anyhow::{
    anyhow,
    Result,
};
use libc::*;
use std::sync::Arc;

use super::{
    media_stream::*,
    media_stream_track::*,
    observer::*,
    rtc_datachannel::*,
    rtc_icecandidate::*,
    rtc_peerconnection_configure::*,
    rtc_session_description::*,
};

#[link(name = "batrachiatc", kind = "static")]
extern "C" {
    fn rtc_run();
    fn rtc_close(peer: *const RawRTCPeerConnection);
    #[allow(improper_ctypes)]
    fn create_rtc_peer_connection(
        config: *const RawRTCPeerConnectionConfigure,
        eventer: *const IObserver,
    ) -> *const RawRTCPeerConnection;
    fn rtc_add_ice_candidate(
        peer: *const RawRTCPeerConnection,
        icecandidate: *const RawRTCIceCandidate,
    ) -> bool;

    fn rtc_add_track(
        peer: *const RawRTCPeerConnection,
        track: *const RawMediaStreamTrack,
        id: *const c_char,
    );

    fn rtc_create_data_channel(
        peer: *const RawRTCPeerConnection,
        label: *const c_char,
        options: *const RawDataChannelOptions,
    ) -> *const RawRTCDataChannel;
}

pub(crate) type RawRTCPeerConnection = c_void;

/// The RTCPeerConnection interface represents a WebRTC connection between
/// the local computer and a remote peer.
///
/// It provides methods to connect to a remote peer, maintain and monitor
/// the connection, and close the connection once it's no longer needed.
pub struct RTCPeerConnection {
    raw: *const RawRTCPeerConnection,
    tracks: Vec<(Arc<MediaStreamTrack>, Arc<MediaStream>)>,
}

unsafe impl Send for RTCPeerConnection {}
unsafe impl Sync for RTCPeerConnection {}

impl RTCPeerConnection {
    /// By default, RTCPeerConnection::run() calls Thread::Current()->Run().
    /// To receive and dispatch messages, call ProcessMessages occasionally.
    pub fn run() {
        unsafe { rtc_run() }
    }

    /// The RTCPeerConnection constructor returns a newly-created
    /// RTCPeerConnection, which represents a connection between the local
    /// device and a remote peer.
    pub fn new(config: &RTCConfiguration, observer: &Observer) -> Result<Self> {
        let raw = unsafe {
            create_rtc_peer_connection(config.get_raw(), observer.get_raw())
        };
        if raw.is_null() {
            Err(anyhow!("create peerconnection failed!"))
        } else {
            Ok(Self {
                raw: unsafe { &*raw },
                tracks: Vec::with_capacity(10),
            })
        }
    }

    /// The create_offer() method of the RTCPeerConnection interface initiates
    /// the creation of an SDP offer for the purpose of starting a new WebRTC
    /// connection to a remote peer. The SDP offer includes information about
    /// any MediaStreamTrack objects already attached to the WebRTC session,
    /// codec, and options supported by the browser, and any candidates already
    /// gathered by the ICE agent, for the purpose of being sent over the
    /// signaling channel to a potential peer to request a connection or to
    /// update the configuration of an existing connection.
    pub fn create_offer(&self) -> CreateDescriptionFuture {
        CreateDescriptionFuture::new(self.raw, CreateDescriptionKind::Offer)
    }

    /// The create_answer() method on the RTCPeerConnection interface creates an
    /// SDP answer to an offer received from a remote peer during the
    /// offer/answer negotiation of a WebRTC connection. The answer contains
    /// information about any media already attached to the session, codecs and
    /// options supported by the browser, and any ICE candidates already
    /// gathered. The answer is delivered to the returned Future, and should
    /// then be sent to the source of the offer to continue the negotiation
    /// process.
    pub fn create_answer(&self) -> CreateDescriptionFuture {
        CreateDescriptionFuture::new(self.raw, CreateDescriptionKind::Answer)
    }

    /// The RTCPeerConnection method setLocalDescription() changes the local
    /// description associated with the connection. This description specifies
    /// the properties of the local end of the connection, including the media
    /// format.
    pub fn set_local_description<'b>(
        &'b self,
        desc: &'b RTCSessionDescription,
    ) -> SetDescriptionFuture<'b> {
        SetDescriptionFuture::new(self.raw, desc, SetDescriptionKind::Local)
    }

    /// The RTCPeerConnection method setRemoteDescription() sets the specified
    /// session description as the remote peer's current offer or answer. The
    /// description specifies the properties of the remote end of the
    /// connection, including the media format.
    pub fn set_remote_description<'b>(
        &'b self,
        desc: &'b RTCSessionDescription,
    ) -> SetDescriptionFuture<'b> {
        SetDescriptionFuture::new(self.raw, desc, SetDescriptionKind::Remote)
    }

    /// When a web site or app using RTCPeerConnection receives a new ICE
    /// candidate from the remote peer over its signaling channel, it
    /// delivers the newly-received candidate to the browser's ICE agent by
    /// calling RTCPeerConnection.addIceCandidate(). This adds this new
    /// remote candidate to the RTCPeerConnection's remote description,
    /// which describes the state of the remote end of the connection.
    ///
    /// If the candidate parameter is missing or a value of null is given when
    /// calling addIceCandidate(), the added ICE candidate is an
    /// "end-of-candidates" indicator. The same is the case if the value of
    /// the specified object's candidate is either missing or an empty
    /// string (""), it signals that all remote candidates have been
    /// delivered.
    ///
    /// The end-of-candidates notification is transmitted to the remote peer
    /// using a candidate with an a-line value of end-of-candidates.
    ///
    /// During negotiation, your app will likely receive many candidates which
    /// you'll deliver to the ICE agent in this way, allowing it to build up
    /// a list of potential connection methods. This is covered in more
    /// detail in the articles WebRTC connectivity and Signaling and video
    /// calling.
    pub fn add_ice_candidate<'b>(
        &'b self,
        candidate: &'b RTCIceCandidate,
    ) -> Result<()> {
        let raw: RawRTCIceCandidate = candidate.try_into()?;
        let ret = unsafe { rtc_add_ice_candidate(self.raw, &raw) };
        if !ret {
            return Err(anyhow!("add ice candidate failed!"));
        }

        Ok(())
    }

    /// The RTCPeerConnection method addTrack() adds a new media track to the
    /// set of tracks which will be transmitted to the other peer.
    pub fn add_track(
        &mut self,
        track: Arc<MediaStreamTrack>,
        stream: Arc<MediaStream>,
    ) {
        unsafe { rtc_add_track(self.raw, track.get_raw(), stream.get_id()) }
        self.tracks.push((track, stream));
    }

    /// The createDataChannel() method on the RTCPeerConnection interface
    /// creates a new channel linked with the remote peer, over which any kind
    /// of data may be transmitted.
    pub fn create_data_channel(
        &self,
        label: &str,
        opt: &DataChannelOptions,
    ) -> Arc<RTCDataChannel> {
        let c_label = to_c_str(label).unwrap();
        let opt: RawDataChannelOptions = opt.into();
        let raw = unsafe { rtc_create_data_channel(self.raw, c_label, &opt) };
        free_cstring(c_label);
        RTCDataChannel::from_raw(raw)
    }
}

impl Drop for RTCPeerConnection {
    fn drop(&mut self) {
        unsafe { rtc_close(self.raw) }
    }
}
