use std::marker::PhantomData;
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

pub struct BitcodeCodec<In, Out> {
    inner: LengthDelimitedCodec,
    _types: PhantomData<(In, Out)>,
}
