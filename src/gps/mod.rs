//! GPS / QZSS protocol

/// GPS uses 30 bit data words, and is not aligned.
pub const GPS_WORD_SIZE: usize = 30;

/// Minimal allocation to encode a correct GPS data frame.
pub const GPS_MIN_SIZE: usize = GPS_WORD_SIZE * 10;

mod bytes;
mod decoder;
mod errors;
mod frame1;
mod frame2;
mod frame3;
mod frame_id;
mod how;
mod tlm;

pub use bytes::GpsDataByte;
pub use decoder::GpsQzssDecoder;
pub use errors::GpsError;
pub use frame_id::GpsQzssFrameId;
pub use how::GpsQzssHow;
pub use tlm::GpsQzssTelemetry;

pub use frame1::GpsQzssFrame1;
pub use frame2::GpsQzssFrame2;
pub use frame3::GpsQzssFrame3;

/// GPS / QZSS interpreted frame.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GpsQzssFrame {
    /// [GpsQzssHow] describes following frame.
    pub how: GpsQzssHow,

    /// [GpsQzssTelemetry] describes following frame.
    pub telemetry: GpsQzssTelemetry,

    /// [GpsQzssSubframe] depends on associated How.
    pub subframe: GpsQzssSubframe,
}

/// GPS / QZSS Interpreted subframes
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GpsQzssSubframe {
    /// GPS Ephemeris Frame #1
    Ephemeris1(GpsQzssFrame1),

    /// GPS Ephemeris Frame #2
    Ephemeris2(GpsQzssFrame2),

    /// GPS Ephemeris Frame #3
    Ephemeris3(GpsQzssFrame3),
}

impl Default for GpsQzssSubframe {
    fn default() -> Self {
        Self::Ephemeris1(Default::default())
    }
}

impl GpsQzssSubframe {
    /// Unwraps self as [GpsQzssFrame1] reference (if feasible)
    pub fn as_eph1(&self) -> Option<&GpsQzssFrame1> {
        match self {
            Self::Ephemeris1(frame) => Some(frame),
            _ => None,
        }
    }

    /// Unwraps self as mutable [GpsQzssFrame1] reference (if feasible)
    pub fn as_mut_eph1(&mut self) -> Option<&mut GpsQzssFrame1> {
        match self {
            Self::Ephemeris1(frame) => Some(frame),
            _ => None,
        }
    }

    /// Unwraps self as [GpsQzssFrame2] reference (if feasible)
    pub fn as_eph2(&self) -> Option<&GpsQzssFrame2> {
        match self {
            Self::Ephemeris2(frame) => Some(frame),
            _ => None,
        }
    }

    /// Unwraps self as [GpsQzssFrame2] reference (if feasible)
    pub fn as_mut_eph2(&mut self) -> Option<&mut GpsQzssFrame2> {
        match self {
            Self::Ephemeris2(frame) => Some(frame),
            _ => None,
        }
    }

    /// Unwraps self as [GpsQzssFrame3] reference (if feasible)
    pub fn as_eph3(&self) -> Option<&GpsQzssFrame3> {
        match self {
            Self::Ephemeris3(frame) => Some(frame),
            _ => None,
        }
    }

    /// Unwraps self as [GpsQzssFrame3] reference (if feasible)
    pub fn as_mut_eph3(&mut self) -> Option<&mut GpsQzssFrame3> {
        match self {
            Self::Ephemeris3(frame) => Some(frame),
            _ => None,
        }
    }
}

// /// Verifies 24-bit LSB (right aligned) parity
// pub(crate) fn check_parity(value: u32) -> bool {
//     let data = value >> 6;
//     let expected = parity_encoding(data);
//     let parity = (value & 0x3f) as u8;
//
//     if expected == parity {
//         true
//     } else {
//         #[cfg(feature = "log")]
//         error!(
//             "GPS: parity error - expected 0x{:02X} - got 0x{:02X}",
//             expected, parity
//         );
//
//         false
//     }
// }
//
// /// Encodes 24-bit LSB into 6 bit parity (right aligned!)
// pub(crate) fn parity_encoding(value: u32) -> u8 {
//     let generator = 0x61_u32;
//     let mut reg = value << 6;
//
//     for _ in 0..24 {
//         if reg & (1 << 29) != 0 {
//             reg ^= generator << 23;
//         }
//         reg <<= 1;
//     }
//
//     ((reg >> 30) & 0x3f) as u8
// }
