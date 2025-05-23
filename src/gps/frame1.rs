use crate::twos_complement;

const WORD3_WEEK_MASK: u32 = 0x3ff00000;
const WORD3_WEEK_SHIFT: u32 = 20;
const WORD3_CA_P_L2_MASK: u32 = 0x000C0000;
const WORD3_CA_P_L2_SHIFT: u32 = 18;
const WORD3_URA_MASK: u32 = 0x0003C000;
const WORD3_URA_SHIFT: u32 = 14;
const WORD3_HEALTH_MASK: u32 = 0x00003f00;
const WORD3_HEALTH_SHIFT: u32 = 8;
const WORD3_IODC_MASK: u32 = 0x000000c0;
const WORD3_IODC_SHIFT: u32 = 6;

const WORD4_L2P_DATA_MASK: u32 = 0x20000000;
const WORD4_RESERVED_MASK: u32 = 0x1fffffc0;
const WORD4_RESERVED_SHIFT: u32 = 6;

const WORD5_RESERVED_MASK: u32 = 0x3fffffc0;
const WORD5_RESERVED_SHIFT: u32 = 6;

const WORD6_RESERVED_MASK: u32 = 0x3fffffc0;
const WORD6_RESERVED_SHIFT: u32 = 6;

const WORD7_RESERVED_MASK: u32 = 0x3fffc000;
const WORD7_RESERVED_SHIFT: u32 = 14;
const WORD7_TGD_MASK: u32 = 0x00003fc0;
const WORD7_TGD_SHIFT: u32 = 6;

const WORD8_IODC_MASK: u32 = 0x3fc00000;
const WORD8_IODC_SHIFT: u32 = 22;
const WORD8_TOC_MASK: u32 = 0x003fffc0;
const WORD8_TOC_SHIFT: u32 = 6;

const WORD9_AF2_MASK: u32 = 0x3fc00000;
const WORD9_AF2_SHIFT: u32 = 22;
const WORD9_AF1_MASK: u32 = 0x003fffc0;
const WORD9_AF1_SHIFT: u32 = 6;

const WORD10_AF0_MASK: u32 = 0x3fffff00;
const WORD10_AF0_SHIFT: u32 = 8;

/// GPS / QZSS Frame #1 interpretation
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GpsQzssFrame1 {
    /// 10-bit week counter (no rollover compensation).
    pub week: u16,

    /// 2-bit C/A or P ON L2.  
    /// When asserted, indicates the NAV data stream was commanded OFF on the L2 channel P-code.
    pub ca_or_p_l2: u8,

    /// 4-bit URA index. The lower the better, interpret as follow (error in meters)
    /// - 0:  0 < ura <= 2.4m
    /// - 1:  2.4 < ura <= 3.4m
    /// - 2:  3.4 < ura <= 4.85
    /// - 3:  4.85 < ura <= 6.85
    /// - 4:  6.85 < ura <= 9.65
    /// - 5:  9.65 < ura <= 13.65
    /// - 6:  13.65 < ura <= 24.00
    /// - 7:  24.00 < ura <= 48.00
    /// - 8:  48.00 < ura <= 96.00
    /// - 9:  96.00 < ura <= 192.00
    /// - 10: 192.00 < ura <=  384.00
    /// - 11: 384.00 < ura <=  768.00
    /// - 12: 768.00 < ura <= 1536.00
    /// - 13: 1536.00 < ura <= 3072.00
    /// - 14: 3072.00 < ura <= 6144.00
    /// - 15: 6144.00 < ura
    ///
    /// For each URA index, users may compute a nominal URA value (x)
    ///  - ura < 6: 2**(1+N/2)
    ///  - ura > 6: 2**(N-2)
    pub ura: u8,

    /// 6-bit SV Health. 0 means all good.
    pub health: u8,

    /// 10-bit IODC.  
    pub iodc: u16,

    /// Time of clock (in seconds)
    pub toc: u32,

    /// 8-bit TGD (in seconds)
    pub tgd: f64,

    /// af2 (in seconds per s^-^&)
    pub af2: f64,

    /// af1 (in seconds per seconds)
    pub af1: f64,

    /// af0 (in seconds)
    pub af0: f64,

    /// 32-bit reserved word #4
    pub reserved_word4: u32,

    pub l2_p_data_flag: bool,

    /// 24-bit reserved word #5
    pub reserved_word5: u32,

    /// 24-bit reserved word #6
    pub reserved_word6: u32,

    ///16-bit reserved word #7
    pub reserved_word7: u16,
}

impl GpsQzssFrame1 {
    pub fn with_week(mut self, week: u16) -> Self {
        self.week = week;
        self
    }

    pub fn with_health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn with_toc(mut self, toc: u32) -> Self {
        self.toc = toc;
        self
    }

    pub fn with_tgd(mut self, tgd: f64) -> Self {
        self.tgd = tgd;
        self
    }

    pub fn with_ura(mut self, ura: u8) -> Self {
        self.ura = ura;
        self
    }

    pub fn with_af0(mut self, af0: f64) -> Self {
        self.af0 = af0;
        self
    }

    pub fn with_af1(mut self, af1: f64) -> Self {
        self.af1 = af1;
        self
    }

    pub fn with_af2(mut self, af2: f64) -> Self {
        self.af2 = af2;
        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word3 {
    /// 10-bit week counter
    pub week: u16,

    /// 2 bits C/A or P ON L2
    pub ca_or_p_l2: u8,

    /// 4-bit URA index
    pub ura: u8,

    /// 6-bit SV Health
    pub health: u8,

    /// 2-bit (MSB) IODC, you will have to associate this to Word # 8
    pub iodc_msb: u8,
}

impl Word3 {
    pub(crate) fn decode(dword: u32) -> Self {
        let week = ((dword & WORD3_WEEK_MASK) >> WORD3_WEEK_SHIFT) as u16;
        let ca_or_p_l2 = ((dword & WORD3_CA_P_L2_MASK) >> WORD3_CA_P_L2_SHIFT) as u8;
        let ura = ((dword & WORD3_URA_MASK) >> WORD3_URA_SHIFT) as u8;
        let health = ((dword & WORD3_HEALTH_MASK) >> WORD3_HEALTH_SHIFT) as u8;
        let iodc_msb = ((dword & WORD3_IODC_MASK) >> WORD3_IODC_SHIFT) as u8;

        Self {
            week,
            ca_or_p_l2,
            ura,
            health,
            iodc_msb,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word4 {
    pub l2_p_data_flag: bool,
    pub reserved: u32,
}

impl Word4 {
    pub(crate) fn decode(dword: u32) -> Self {
        let l2_p_data_flag = (dword & WORD4_L2P_DATA_MASK) > 0;
        let reserved = ((dword & WORD4_RESERVED_MASK) >> WORD4_RESERVED_SHIFT) as u32;
        Self {
            l2_p_data_flag,
            reserved,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word5 {
    /// 24-bit reserved
    pub reserved: u32,
}

impl Word5 {
    pub(crate) fn decode(dword: u32) -> Self {
        let reserved = (dword & WORD5_RESERVED_MASK) >> WORD5_RESERVED_SHIFT;
        Self { reserved }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word6 {
    /// 24-bit reserved
    pub reserved: u32,
}

impl Word6 {
    pub(crate) fn decode(dword: u32) -> Self {
        let reserved = (dword & WORD6_RESERVED_MASK) >> WORD6_RESERVED_SHIFT;
        Self { reserved }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word7 {
    /// 16-bit reserved
    pub reserved: u16,

    /// TGD
    pub tgd: i8,
}

impl Word7 {
    pub(crate) fn decode(dword: u32) -> Self {
        let reserved = ((dword & WORD7_RESERVED_MASK) >> WORD7_RESERVED_SHIFT) as u16;
        let tgd = ((dword & WORD7_TGD_MASK) >> WORD7_TGD_SHIFT) as i8;
        Self { reserved, tgd }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word8 {
    /// 8-bit IODC LSB to associate with Word # 3
    pub iodc_lsb: u8,

    /// 16 bit ToC
    pub toc: u16,
}

impl Word8 {
    pub(crate) fn decode(dword: u32) -> Self {
        let iodc_lsb = ((dword & WORD8_IODC_MASK) >> WORD8_IODC_SHIFT) as u8;
        let toc = ((dword & WORD8_TOC_MASK) >> WORD8_TOC_SHIFT) as u16;
        Self { iodc_lsb, toc }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word9 {
    /// 8 bit af2
    pub af2: i8,

    /// 16 bit af1
    pub af1: i16,
}

impl Word9 {
    pub(crate) fn decode(dword: u32) -> Self {
        let af2 = ((dword & WORD9_AF2_MASK) >> WORD9_AF2_SHIFT) as i8;
        let af1 = ((dword & WORD9_AF1_MASK) >> WORD9_AF1_SHIFT) as i16;
        Self { af2, af1 }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Word10 {
    /// 22-bit af0
    pub af0: i32,
}

impl Word10 {
    pub(crate) fn decode(dword: u32) -> Self {
        let af0 = ((dword & WORD10_AF0_MASK) >> WORD10_AF0_SHIFT) as u32;
        let af0 = twos_complement(af0, 0x3fffff, 0x200000);
        Self { af0 }
    }
}
