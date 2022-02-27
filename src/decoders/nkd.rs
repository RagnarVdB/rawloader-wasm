use crate::decoders::*;
use crate::decoders::basics::*;
use std::f32::NAN;

//#[derive(Debug, Clone)]
pub struct NakedDecoder<'a> {
  buffer: &'a [u8],
  camera: Camera,
}

impl<'a> NakedDecoder<'a> {
  pub fn new(buf: &'a [u8], cam: Camera, _rawloader: &'a RawLoader) -> NakedDecoder<'a> {
    NakedDecoder {
      buffer: buf,
      camera: cam,
    }
  }
}

impl<'a> Decoder for NakedDecoder<'a> {
  fn image(&self, dummy: bool) -> Result<RawImage,String> {
    let width = self.camera.raw_width;
    let height = self.camera.raw_height;
    let size = self.camera.filesize;
    let bits = size*8 / width / height;

    let image = if self.camera.find_hint("12le_16bitaligned") {
      decode_12le_16bitaligned(self.buffer, width, height, dummy)
    } else {
      match bits {
        10 => decode_10le_lsb16(self.buffer, width, height, dummy),
        12 => decode_12be_msb16(self.buffer, width, height, dummy),
        _  => return Err(format!("Naked: Don't know about {} bps images", bits).to_string()),
      }
    };

    ok_image(self.camera.clone(), width, height, [NAN,NAN,NAN,NAN], 0, 0, image)
  }
}
