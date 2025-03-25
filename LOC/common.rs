pub struct CommonContainer { //TODO: ask if container should contain header and payload or they should be separate
    pub header: CommonHeader,
    pub payload: Vec<u8>,
}

impl CommonContainer {
    pub fn new(header: CommonHeader, payload: Vec<u8>) -> Self {
        return Self { header, payload };
    }
}

pub struct CommonHeader {
    id: CommonHeaderId,
    length: u8, //TODO: can be dropped — it’s redundant since we can compute value.len()
    value: u32, //TODO: what should the data type be?
}

impl CommonHeader {
    pub fn new(id: CommonHeaderId, length: u8, value: u32) -> Self {
        return Self { id, length, value };
    }
}

pub enum CommonHeaderId {
    // Description: Wall-clock time in microseconds since the Unix epoch when the encoded media frame was captured,
    // encoded as a 64 bit unsigned integer in network byte order (big endian).
    // ID: 2 (IANA, please assign from the MOQ Header Extensions Registry)
    // Length: 8 (bytes)
    // Value: Varies
    CaptureTimestamp = 0x2,
}

