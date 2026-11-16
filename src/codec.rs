use std::marker::PhantomData;
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

pub struct BitcodeCodec<In, Out> {
    inner: LengthDelimitedCodec,
    _types: PhantomData<(In, Out)>,
}

impl<In, Out> BitcodeCodec<In, Out> {
    pub fn new() -> Self {
        let inner = LengthDelimitedCodec::builder()
            .max_frame_length(1024 * 1024)
            .new_codec();

        Self {
            inner,
            _types: PhantomData,
        }
    }
}
