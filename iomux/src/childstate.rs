use bytelinebuf::ByteLineBuf;

pub struct ChildState {
    pub pid: u32,
    pub outbuf: ByteLineBuf,
    pub errbuf: ByteLineBuf,
}

impl ChildState {
    pub fn new(pid: u32) -> Self {
        ChildState {
            pid,
            outbuf: ByteLineBuf::default(),
            errbuf: ByteLineBuf::default(),
        }
    }
}
