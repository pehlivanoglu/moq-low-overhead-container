pub struct AudioContainer { //TODO: ask if container should contain header and payload or they should be separate
    pub header: AudioHeader,
    pub payload: Vec<u8>,
}

impl AudioContainer {
    pub fn new(header: AudioHeader, payload: Vec<u8>) -> Self {
        return Self { header, payload };
    }
}

pub struct AudioHeader {
    id: AudioHeaderId,
    length: u8, //TODO: can be dropped — it’s redundant since we can compute value.len()
    value: u32, //TODO: what should the data type be?
}

impl AudioHeader {
    pub fn new(id: AudioHeaderId, length: u8, value: u32) -> Self {
        return Self { id, length, value };
    }
}

pub enum AudioHeaderId {
    // Description: The magnitude of the audio level of the corresponding audio frame encoded in 7 bits as defined in section 3 of [RFC6464].
    // ID: 6 (IANA, please assign from the MOQ Header Extensions Registry)
    // Length: 1 (byte)
    // Value: Varies
    AudioLevel = 0x6,
}