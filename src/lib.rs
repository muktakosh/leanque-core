//! Core library for leanque
//!
//! This library provides communication primitives used by the leanque stack.
//! Every other library/service/application using leanque hovers around these
//! primitives.

use std::fmt;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum PeerID {
    Local(String),
    Remote(String, String),
}

impl fmt::Display for PeerID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PeerID::Local(ref id) => write!(f, "{}@{}", id, "localhost"),
            PeerID::Remote(ref id, ref hostname) => write!(f, "{}@{}", id, hostname),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PeerStatus {
    NotReady,
    Open,
    InputOnly,
    OutputOnly,
    Closed,
    Error,
}

impl fmt::Display for PeerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   PeerStatus::NotReady => "NotReady",
                   PeerStatus::Open => "Open",
                   PeerStatus::InputOnly => "InputOnly",
                   PeerStatus::OutputOnly => "OutputOnly",
                   PeerStatus::Closed => "Closed",
                   PeerStatus::Error => "Error",
               })
    }
}

#[derive(Clone)]
pub struct PeerConnection {
    peer: PeerID,
    status: PeerStatus,
}

impl PeerConnection {
    pub fn new(peer: PeerID) -> Self {
        PeerConnection {
            peer: peer,
            status: PeerStatus::NotReady,
        }
    }

    pub fn change_status(&mut self, status: PeerStatus) {
        self.status = status;
    }
}

#[derive(Clone)]
pub struct Peer {
    id: PeerID,
    status: PeerStatus,
    connections: HashMap<String, PeerConnection>,
}

impl Peer {
    pub fn local(id: String) -> Self {
        Peer::new(PeerID::Local(id))
    }

    pub fn remote(id: String, hostname: String) -> Self {
        Peer::new(PeerID::Remote(id, hostname))
    }

    pub fn process(&mut self) {}

    pub fn open(&mut self) {
        self.status = PeerStatus::Open;
    }

    pub fn is_open(&self) -> bool {
        match self.status {
            PeerStatus::Open | PeerStatus::InputOnly | PeerStatus::OutputOnly => true,
            _ => false,
        }
    }

    pub fn close(&mut self) {
        self.status = PeerStatus::Closed;
    }

    pub fn connect(&mut self, peer: PeerID) {
        let peer_string = peer.to_string();
        self.connections.entry(peer_string.clone()).or_insert(PeerConnection::new(peer));
        if self.is_open() {
            self.create_peer_connection(peer_string);
        }
    }

    pub fn disconnect(&mut self) {}

    pub fn recv(&self) {}

    pub fn send(&self) {}
}

impl Peer {
    fn new(id: PeerID) -> Self {
        Peer {
            id: id,
            status: PeerStatus::NotReady,
            connections: HashMap::new(),
        }
    }

    fn create_peer_connection(&mut self, peer_id: String) {}
}
