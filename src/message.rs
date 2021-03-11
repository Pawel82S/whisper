use std::net::IpAddr;

pub struct Message {
    contents: String,
}

impl Message {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }

    pub fn send(&self, target: IpAddr) -> bool {
        unimplemented!()
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    /// Will panic if seed is shorter than message.
    pub fn enc_dec(self, seed: &[u8]) -> String {
        let length = self.contents.len();

        if seed.len() < length {
            panic!("Encryption/Decryption seed isn't long as message");
        }

        let mut result = Vec::with_capacity(length);
        let mut index = 0;

        for byte in self.contents.as_bytes() {
            let ch = byte ^ seed[index];
            result.push(ch as char);
            index += 1;
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_message() {
        let message = Message::new("Hello World".to_string());
        assert_eq!(message.contents(), "Hello World");
    }

    #[test]
    fn encryption() {
        let message = Message::new("Hello World".to_string());
    }
}
