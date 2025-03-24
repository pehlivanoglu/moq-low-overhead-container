pub struct CommonContainer {
    pub header: CommonHeader,
    pub payload: Vec<u8>, //TODO: implement payload
}

pub struct CommonHeader {
    value: u32,
    id: CommonHeaderId,
}

pub enum CommonHeaderId {
    // Description: Wall-clock time in microseconds since the Unix epoch when the encoded media frame was captured,
    // encoded as a 64 bit unsigned integer in network byte order (big endian).
    // ID: 2 (IANA, please assign from the MOQ Header Extensions Registry)
    // Length: 8 (bytes)
    // Value: Varies
    CaptureTimestamp = 0x2,
}