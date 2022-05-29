use crate::{
    coder::{EncoderApi, EncoderCfg},
    hw, HW_STRIDE_ALIGN,
};
use hbb_common::{
    anyhow::{anyhow, Context},
    lazy_static, log,
    message_proto::{H264s, H265s, Message, VideoFrame, H264, H265},
    ResultType,
};
use hwcodec::{
    decode::{DecodeContext, DecodeFrame, Decoder},
    encode::{EncodeContext, EncodeFrame, Encoder},
    ffmpeg::{CodecInfo, DataFormat},
    AVPixelFormat,
};
use std::sync::{Arc, Mutex, Once};

lazy_static::lazy_static! {
    static ref HW_ENCODER_NAME: Arc<Mutex<Option<String>>> = Default::default();
    static ref HW_DECODER_INSTANCE: Arc<Mutex<HwDecoderInstance>> = Arc::new(Mutex::new(HwDecoderInstance {
        h264: None,
        h265: None,
    }));
}

const DEFAULT_PIXFMT: AVPixelFormat = AVPixelFormat::AV_PIX_FMT_YUV420P;

pub struct HwEncoder {
    encoder: Encoder,
    yuv: Vec<u8>,
    pub format: DataFormat,
    pub pixfmt: AVPixelFormat,
}

impl EncoderApi for HwEncoder {
    fn new(cfg: EncoderCfg) -> ResultType<Self>
    where
        Self: Sized,
    {
        match cfg {
            EncoderCfg::HW(config) => {
                let ctx = EncodeContext {
                    name: config.codec_name.clone(),
                    fps: config.fps as _,
                    width: config.width as _,
                    height: config.height as _,
                    pixfmt: DEFAULT_PIXFMT,
                    align: HW_STRIDE_ALIGN as _,
                };
                let format = match Encoder::format_from_name(config.codec_name.clone()) {
                    Ok(format) => format,
                    Err(_) => {
                        return Err(anyhow!(format!(
                            "failed to get format from name:{}",
                            config.codec_name
                        )))
                    }
                };
                match Encoder::new(ctx.clone()) {
                    Ok(encoder) => Ok(HwEncoder {
                        encoder,
                        yuv: vec![],
                        format,
                        pixfmt: ctx.pixfmt,
                    }),
                    Err(_) => Err(anyhow!(format!("Failed to create encoder"))),
                }
            }
            _ => Err(anyhow!("encoder type mismatch")),
        }
    }

    fn encode_to_message(
        &mut self,
        frame: &[u8],
        _ms: i64,
    ) -> ResultType<hbb_common::message_proto::Message> {
        let mut msg_out = Message::new();
        let mut vf = VideoFrame::new();
        match self.format {
            DataFormat::H264 => {
                let mut h264s = Vec::new();
                for frame in self.encode(frame).with_context(|| "Failed to encode")? {
                    h264s.push(H264 {
                        data: frame.data,
                        pts: frame.pts as _,
                        ..Default::default()
                    });
                }
                if h264s.len() > 0 {
                    vf.set_h264s(H264s {
                        h264s: h264s.into(),
                        ..Default::default()
                    });
                    msg_out.set_video_frame(vf);
                    Ok(msg_out)
                } else {
                    Err(anyhow!("no valid frame"))
                }
            }
            DataFormat::H265 => {
                let mut h265s = Vec::new();
                for frame in self.encode(frame).with_context(|| "Failed to encode")? {
                    h265s.push(H265 {
                        data: frame.data,
                        pts: frame.pts,
                        ..Default::default()
                    });
                }
                if h265s.len() > 0 {
                    vf.set_h265s(H265s {
                        h265s,
                        ..Default::default()
                    });
                    msg_out.set_video_frame(vf);
                    Ok(msg_out)
                } else {
                    Err(anyhow!("no valid frame"))
                }
            }
        }
    }

    fn use_yuv(&self) -> bool {
        false
    }
}

impl HwEncoder {
    pub fn best() -> (Option<CodecInfo>, Option<CodecInfo>) {
        let ctx = EncodeContext {
            name: String::from(""),
            fps: 30,
            width: 1920,
            height: 1080,
            pixfmt: DEFAULT_PIXFMT,
            align: HW_STRIDE_ALIGN as _,
        };
        CodecInfo::score(Encoder::avaliable_encoders(ctx))
    }

    pub fn current_name() -> Arc<Mutex<Option<String>>> {
        HW_ENCODER_NAME.clone()
    }

    pub fn encode(&mut self, bgra: &[u8]) -> ResultType<Vec<EncodeFrame>> {
        match self.pixfmt {
            AVPixelFormat::AV_PIX_FMT_YUV420P => hw::hw_bgra_to_i420(
                self.encoder.ctx.width as _,
                self.encoder.ctx.height as _,
                &self.encoder.linesize,
                &self.encoder.offset,
                self.encoder.length,
                bgra,
                &mut self.yuv,
            ),
            AVPixelFormat::AV_PIX_FMT_NV12 => hw::hw_bgra_to_nv12(
                self.encoder.ctx.width as _,
                self.encoder.ctx.height as _,
                &self.encoder.linesize,
                &self.encoder.offset,
                self.encoder.length,
                bgra,
                &mut self.yuv,
            ),
        }

        match self.encoder.encode(&self.yuv) {
            Ok(v) => {
                let mut data = Vec::<EncodeFrame>::new();
                data.append(v);
                Ok(data)
            }
            Err(_) => Ok(Vec::<EncodeFrame>::new()),
        }
    }
}

pub struct HwDecoder {
    decoder: Decoder,
    pub info: CodecInfo,
}

pub struct HwDecoderInstance {
    pub h264: Option<HwDecoder>,
    pub h265: Option<HwDecoder>,
}

impl HwDecoder {
    pub fn instance() -> Arc<Mutex<HwDecoderInstance>> {
        static ONCE: Once = Once::new();
        // TODO: different process
        ONCE.call_once(|| {
            let avaliable = Decoder::avaliable_decoders();
            let mut decoders = vec![];
            for decoder in avaliable {
                if let Ok(d) = HwDecoder::new(decoder) {
                    decoders.push(d);
                }
            }

            let mut h264: Option<HwDecoder> = None;
            let mut h265: Option<HwDecoder> = None;
            for decoder in decoders {
                match decoder.info.format {
                    DataFormat::H264 => match &h264 {
                        Some(old) => {
                            if decoder.info.score > old.info.score {
                                h264 = Some(decoder)
                            }
                        }
                        None => h264 = Some(decoder),
                    },
                    DataFormat::H265 => match &h265 {
                        Some(old) => {
                            if decoder.info.score > old.info.score {
                                h265 = Some(decoder)
                            }
                        }
                        None => h265 = Some(decoder),
                    },
                }
            }
            if h264.is_some() {
                log::info!("h264 decoder:{:?}", h264.as_ref().unwrap().info);
            }
            if h265.is_some() {
                log::info!("h265 decoder:{:?}", h265.as_ref().unwrap().info);
            }
            HW_DECODER_INSTANCE.lock().unwrap().h264 = h264;
            HW_DECODER_INSTANCE.lock().unwrap().h265 = h265;
        });
        HW_DECODER_INSTANCE.clone()
    }

    pub fn new(info: CodecInfo) -> ResultType<Self> {
        let ctx = DecodeContext {
            name: info.name.clone(),
            device_type: info.hwdevice.clone(),
        };
        match Decoder::new(ctx) {
            Ok(decoder) => Ok(HwDecoder { decoder, info }),
            Err(_) => Err(anyhow!(format!("Failed to create decoder"))),
        }
    }
    pub fn decode(&mut self, data: &[u8]) -> ResultType<Vec<HwDecoderImage>> {
        match self.decoder.decode(data) {
            Ok(v) => Ok(v.iter().map(|f| HwDecoderImage { frame: f }).collect()),
            Err(_) => Ok(vec![]),
        }
    }
}

pub struct HwDecoderImage<'a> {
    frame: &'a DecodeFrame,
}

impl HwDecoderImage<'_> {
    pub fn bgra(&self, bgra: &mut Vec<u8>, i420: &mut Vec<u8>) -> ResultType<()> {
        let frame = self.frame;
        match frame.pixfmt {
            AVPixelFormat::AV_PIX_FMT_NV12 => hw::hw_nv12_to_bgra(
                frame.width as _,
                frame.height as _,
                &frame.data[0],
                &frame.data[1],
                frame.linesize[0] as _,
                frame.linesize[1] as _,
                bgra,
                i420,
                HW_STRIDE_ALIGN,
            ),
            AVPixelFormat::AV_PIX_FMT_YUV420P => {
                hw::hw_i420_to_bgra(
                    frame.width as _,
                    frame.height as _,
                    &frame.data[0],
                    &frame.data[1],
                    &frame.data[2],
                    frame.linesize[0] as _,
                    frame.linesize[1] as _,
                    frame.linesize[2] as _,
                    bgra,
                );
                return Ok(());
            }
        }
    }
}
