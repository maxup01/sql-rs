use bitcode::{Decode, Encode};
use bytes::Bytes;
use std::io;
use std::marker::PhantomData;
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

pub struct BitcodeCodec<T> {
    inner: LengthDelimitedCodec,
    _types: PhantomData<T>,
}

impl<T> BitcodeCodec<T> {
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

impl<T: for<'a> Decode<'a>> Decoder for BitcodeCodec<T> {
    type Item = T;
    type Error = io::Error;

    fn decode(&mut self, src: &mut tokio_util::bytes::BytesMut) -> io::Result<Option<Self::Item>> {
        match self.inner.decode(src)? {
            Some(frame) => bitcode::decode(&frame)
                .map(Some)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            None => Ok(None),
        }
    }
}

impl<T: Encode> Encoder<T> for BitcodeCodec<T> {
    type Error = io::Error;

    fn encode(&mut self, item: T, dst: &mut tokio_util::bytes::BytesMut) -> io::Result<()> {
        self.inner.encode(Bytes::from(bitcode::encode(&item)), dst)
    }
}
