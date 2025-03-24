pub struct AudioContainer {
    pub header: AudioHeader,
    pub payload: Vec<u8>, //TODO: implement payload
}

pub struct AudioHeader {
    value: u32, // TODO: ??
    id: AudioHeaderId,
}

pub enum AudioHeaderId {
    // Description: The magnitude of the audio level of the corresponding audio frame encoded in 7 bits as defined in section 3 of [RFC6464].
    // ID: 6 (IANA, please assign from the MOQ Header Extensions Registry)
    // Length: 1 (byte)
    // Value: Varies
    AudioLevel = 0x6,
}