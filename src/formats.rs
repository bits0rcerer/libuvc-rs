use norm_uvc_sys::*;

#[derive(Debug, Copy, Clone)]
/// Format one can request a stream to produce
pub struct StreamFormat {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub format: FrameFormat,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Format of a frame
pub enum FrameFormat {
    Unknown,
    Any,
    Uncompressed,
    Compressed,
    YUYV,
    UYVY,
    RGB,
    BGR,
    MJPEG,
    H264,
    GRAY8,
    GRAY16,
    BY8,
    BA81,
    SGRBG8,
    SGBRG8,
    SRGGB8,
    SBGGR8,
    NV12,
    P010,
    Count,
}

#[allow(non_upper_case_globals, unreachable_patterns)]
impl From<uvc_frame_format> for FrameFormat {
    fn from(code: uvc_frame_format) -> Self {
        match code {
            uvc_frame_format_UVC_FRAME_FORMAT_ANY => FrameFormat::Any,
            uvc_frame_format_UVC_FRAME_FORMAT_UNCOMPRESSED => FrameFormat::Uncompressed,
            uvc_frame_format_UVC_FRAME_FORMAT_COMPRESSED => FrameFormat::Compressed,
            uvc_frame_format_UVC_FRAME_FORMAT_YUYV => FrameFormat::YUYV,
            uvc_frame_format_UVC_FRAME_FORMAT_UYVY => FrameFormat::UYVY,
            uvc_frame_format_UVC_FRAME_FORMAT_RGB => FrameFormat::RGB,
            uvc_frame_format_UVC_FRAME_FORMAT_BGR => FrameFormat::BGR,
            uvc_frame_format_UVC_FRAME_FORMAT_MJPEG => FrameFormat::MJPEG,
            uvc_frame_format_UVC_FRAME_FORMAT_H264 => FrameFormat::H264,
            uvc_frame_format_UVC_FRAME_FORMAT_GRAY8 => FrameFormat::GRAY8,
            uvc_frame_format_UVC_FRAME_FORMAT_GRAY16 => FrameFormat::GRAY16,
            uvc_frame_format_UVC_FRAME_FORMAT_BY8 => FrameFormat::BY8,
            uvc_frame_format_UVC_FRAME_FORMAT_BA81 => FrameFormat::BA81,
            uvc_frame_format_UVC_FRAME_FORMAT_SGRBG8 => FrameFormat::SGRBG8,
            uvc_frame_format_UVC_FRAME_FORMAT_SGBRG8 => FrameFormat::SGBRG8,
            uvc_frame_format_UVC_FRAME_FORMAT_SRGGB8 => FrameFormat::SRGGB8,
            uvc_frame_format_UVC_FRAME_FORMAT_SBGGR8 => FrameFormat::SBGGR8,
            uvc_frame_format_UVC_FRAME_FORMAT_NV12 => FrameFormat::NV12,
            uvc_frame_format_UVC_FRAME_FORMAT_P010 => FrameFormat::P010,

            uvc_frame_format_UVC_FRAME_FORMAT_COUNT => FrameFormat::Count,
            uvc_frame_format_UVC_FRAME_FORMAT_UNKNOWN => FrameFormat::Unknown, // unreachable
            _ => FrameFormat::Unknown,
        }
    }
}

impl Into<uvc_frame_format> for FrameFormat {
    fn into(self: FrameFormat) -> uvc_frame_format {
        match self {
            FrameFormat::Any => uvc_frame_format_UVC_FRAME_FORMAT_ANY,
            FrameFormat::Uncompressed => uvc_frame_format_UVC_FRAME_FORMAT_UNCOMPRESSED,
            FrameFormat::Compressed => uvc_frame_format_UVC_FRAME_FORMAT_COMPRESSED,
            FrameFormat::YUYV => uvc_frame_format_UVC_FRAME_FORMAT_YUYV,
            FrameFormat::UYVY => uvc_frame_format_UVC_FRAME_FORMAT_UYVY,
            FrameFormat::RGB => uvc_frame_format_UVC_FRAME_FORMAT_RGB,
            FrameFormat::BGR => uvc_frame_format_UVC_FRAME_FORMAT_BGR,
            FrameFormat::MJPEG => uvc_frame_format_UVC_FRAME_FORMAT_MJPEG,
            FrameFormat::H264 => uvc_frame_format_UVC_FRAME_FORMAT_H264,
            FrameFormat::GRAY8 => uvc_frame_format_UVC_FRAME_FORMAT_GRAY8,
            FrameFormat::GRAY16 => uvc_frame_format_UVC_FRAME_FORMAT_GRAY16,
            FrameFormat::BY8 => uvc_frame_format_UVC_FRAME_FORMAT_BY8,
            FrameFormat::BA81 => uvc_frame_format_UVC_FRAME_FORMAT_BA81,
            FrameFormat::SGRBG8 => uvc_frame_format_UVC_FRAME_FORMAT_SGRBG8,
            FrameFormat::SGBRG8 => uvc_frame_format_UVC_FRAME_FORMAT_SGBRG8,
            FrameFormat::SRGGB8 => uvc_frame_format_UVC_FRAME_FORMAT_SRGGB8,
            FrameFormat::SBGGR8 => uvc_frame_format_UVC_FRAME_FORMAT_SBGGR8,
            FrameFormat::NV12 => uvc_frame_format_UVC_FRAME_FORMAT_NV12,
            FrameFormat::P010 => uvc_frame_format_UVC_FRAME_FORMAT_P010,
            FrameFormat::Count => uvc_frame_format_UVC_FRAME_FORMAT_COUNT,
            FrameFormat::Unknown => uvc_frame_format_UVC_FRAME_FORMAT_UNKNOWN,
        }
    }
}
