pub struct VideoContainer { //TODO: ask if container should contain header and payload or they should be separate
    pub header: VideoHeader,
    pub payload: Vec<u8>,
}

impl VideoContainer {
    pub fn new(header: VideoHeader, payload: Vec<u8>) -> Self {
        return Self { header, payload };
    }
}

pub struct VideoHeader {
    id: VideoHeaderId,
    length: u8, //TODO: can be dropped — it’s redundant since we can compute value.len()
    value: u32, //TODO: what should the data type be?
    
}

impl VideoHeader {
    pub fn new(id: VideoHeaderId, length: u8, value: u32) -> Self {
        return Self { id, length, value };
    }
}

pub enum VideoHeaderId {
    // Description: Flags for video frames which are independent, discardable, or base layer sync points,
    // as well as temporal and spatial layer identification, as defined in [Framemarking].
    // ID: 4 (IANA, please assign from the MOQ Header Extensions Registry)
    // Length: Varies (1-3 bytes)
    // Value: Varies
    VideoFrameMarking = 0x4,
    
    // Description: Video codec configuration "extradata", as defined by the corresponding codec specification,
    // which maps to the WebCodecs VideoDecoderConfig description property in the EncodedVideoChunkMetadata.
    // ID: 16 (IANA, please assign from the MOQ Header Extensions Registry)
    // Length: Varies
    // Value: Varies
    VideoConfig = 0x10,
}