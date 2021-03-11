//! The louder you speak the more silent you become.
//!
//! This library is part of open source project dedicated to encrypting P2P communication between
//! 2 people providing perfect secrecy of communication between two people arouound the world.
//! It uses One Time Pass for communication between two peers and distribution of OTP between
//! them in semless way. Because privacy shouldn't be an option. It should be default.

mod message;

use message::Message;

use std::net::IpAddr;

pub fn send_message(msg: Message, target: IpAddr) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
