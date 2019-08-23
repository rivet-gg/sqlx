use super::Decode;
use std::convert::TryInto;

#[derive(Debug)]
pub struct BackendKeyData {
    /// The process ID of this backend.
    process_id: u32,

    /// The secret key of this backend.
    secret_key: u32,
}

impl BackendKeyData {
    #[inline]
    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    #[inline]
    pub fn secret_key(&self) -> u32 {
        self.secret_key
    }
}

impl Decode for BackendKeyData {
    fn decode(src: &[u8]) -> Self {
        debug_assert_eq!(src.len(), 8);

        let process_id = u32::from_be_bytes(src[0..4].try_into().unwrap());
        let secret_key = u32::from_be_bytes(src[4..8].try_into().unwrap());

        Self {
            process_id,
            secret_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BackendKeyData, Decode};
    use bytes::Bytes;

    const BACKEND_KEY_DATA: &[u8] = b"\0\0'\xc6\x89R\xc5+";

    #[test]
    fn it_decodes_backend_key_data() {
        let message = BackendKeyData::decode(BACKEND_KEY_DATA);

        assert_eq!(message.process_id(), 10182);
        assert_eq!(message.secret_key(), 2303903019);
    }
}