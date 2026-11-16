use bitcode::{Decode, Encode};
use bytes::{Bytes, BytesMut};
use std::io;
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

impl<In: for<'a> Decode<'a>, Out> Decoder for BitcodeCodec<In, Out> {
    type Item = In;
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

impl<In, Out: Encode> Encoder<Out> for BitcodeCodec<In, Out> {
    type Error = io::Error;

    fn encode(&mut self, item: Out, dst: &mut tokio_util::bytes::BytesMut) -> io::Result<()> {
        self.inner.encode(Bytes::from(bitcode::encode(&item)), dst)
    }
}
