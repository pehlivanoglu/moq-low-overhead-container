pub struct VideoContainer {
    pub header: VideoHeader,
    pub payload: Vec<u8>, //TODO: implement payload
}

pub struct VideoHeader {
    value: u32, // TODO: ??
    id: VideoHeaderId,
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