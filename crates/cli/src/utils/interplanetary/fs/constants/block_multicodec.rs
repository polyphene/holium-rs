#[derive(Clone, Copy)]
/// Enumeration of multicodecs used in the Holium framework.
/// Reference: https://github.com/multiformats/multicodec
pub enum BlockMulticodec {
    Raw,
    DagCbor,
}


impl From<&BlockMulticodec> for u64 {
    fn from(codec: &BlockMulticodec) -> Self {
        match codec {
            BlockMulticodec::Raw => 0x55,
            BlockMulticodec::DagCbor => 0x71,
        }
    }
}