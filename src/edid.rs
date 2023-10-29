pub struct Edid<'a>(
    pub(crate) *const display_info_sys::di_edid,
    pub(crate) std::marker::PhantomData<&'a ()>,
);

pub type EdidChromaticityCoords = display_info_sys::di_edid_chromaticity_coords;
pub type EdidColorEncodingFormats = display_info_sys::di_edid_color_encoding_formats;

impl<'a> Edid<'a> {
    pub fn basic_gamma(&self) -> Option<f32> {
        let gamma = unsafe { display_info_sys::di_edid_get_basic_gamma(self.0) };
        if gamma == 0.0 {
            None
        } else {
            Some(gamma)
        }
    }

    pub fn chromaticity_coords(&self) -> &'a EdidChromaticityCoords {
        unsafe { &*display_info_sys::di_edid_get_chromaticity_coords(self.0) }
    }

    pub fn color_encoding_formats(&self) -> &'a EdidColorEncodingFormats {
        unsafe { &*display_info_sys::di_edid_get_color_encoding_formats(self.0) }
    }

    pub fn detailed_timings(&self) -> &[DetailedTiming<'a>] {
        let start = unsafe { display_info_sys::di_edid_get_detailed_timing_defs(self.0) };
        let mut len = 0;
        let mut i = unsafe { *start };
        while !i.is_null() {
            len += 1;
            i = unsafe { i.offset(1) };
        }

        unsafe { std::slice::from_raw_parts(start as *const DetailedTiming<'a>, len) }
    }
}

pub type AnalogComposite = display_info_sys::di_edid_detailed_timing_analog_composite;
pub type BipolarAnalogComposite =
    display_info_sys::di_edid_detailed_timing_bipolar_analog_composite;

#[repr(C)]
pub struct DigitalComposite {
    sync_serrations: bool,
    sync_horiz_polarity: Polarity,
}

#[repr(C)]
pub struct DigitalSeparate {
    sync_vert_polarity: Polarity,
    sync_horiz_polarity: Polarity,
}

#[repr(C)]
pub enum Polarity {
    Negative = 0,
    Positive = 1,
}

#[repr(transparent)]
pub struct DetailedTiming<'a>(
    *const display_info_sys::di_edid_detailed_timing_def,
    std::marker::PhantomData<&'a ()>,
);

impl<'a> DetailedTiming<'a> {
    pub fn pixel_clock_hz(&self) -> i32 {
        unsafe { *self.0 }.pixel_clock_hz
    }

    pub fn horiz_video(&self) -> i32 {
        unsafe { *self.0 }.horiz_video
    }

    pub fn vert_video(&self) -> i32 {
        unsafe { *self.0 }.vert_video
    }

    pub fn horiz_blank(&self) -> i32 {
        unsafe { *self.0 }.horiz_blank
    }

    pub fn vert_blank(&self) -> i32 {
        unsafe { *self.0 }.vert_blank
    }

    pub fn horiz_front_porch(&self) -> i32 {
        unsafe { *self.0 }.horiz_front_porch
    }

    pub fn vert_front_porch(&self) -> i32 {
        unsafe { *self.0 }.vert_front_porch
    }

    pub fn horiz_sync_pulse(&self) -> i32 {
        unsafe { *self.0 }.horiz_sync_pulse
    }

    pub fn vert_sync_pulse(&self) -> i32 {
        unsafe { *self.0 }.vert_sync_pulse
    }

    pub fn horiz_image_mm(&self) -> i32 {
        unsafe { *self.0 }.horiz_image_mm
    }

    pub fn vert_image_mm(&self) -> i32 {
        unsafe { *self.0 }.vert_image_mm
    }

    pub fn horiz_border(&self) -> i32 {
        unsafe { *self.0 }.horiz_border
    }

    pub fn vert_border(&self) -> i32 {
        unsafe { *self.0 }.vert_border
    }

    pub fn interlaced(&self) -> bool {
        unsafe { *self.0 }.interlaced
    }

    pub fn stereo(&self) -> Stereo {
        Stereo::from(unsafe { *self.0 }.stereo)
    }

    pub fn signal(&self) -> Signal {
        match unsafe { *self.0 }.signal_type {
            display_info_sys::di_edid_detailed_timing_def_signal_type::DI_EDID_DETAILED_TIMING_DEF_SIGNAL_ANALOG_COMPOSITE => Signal::AnalogComposite(
                unsafe { &*(*self.0).analog_composite }
            ),
            display_info_sys::di_edid_detailed_timing_def_signal_type::DI_EDID_DETAILED_TIMING_DEF_SIGNAL_BIPOLAR_ANALOG_COMPOSITE => Signal::BipolarAnalogComposite(
                unsafe { &*(*self.0).bipolar_analog_composite }
            ),
            display_info_sys::di_edid_detailed_timing_def_signal_type::DI_EDID_DETAILED_TIMING_DEF_SIGNAL_DIGITAL_COMPOSITE => Signal::DigitalComposite(
                unsafe { std::mem::transmute(&*(*self.0).digital_composite) }
            ),
            display_info_sys::di_edid_detailed_timing_def_signal_type::DI_EDID_DETAILED_TIMING_DEF_SIGNAL_DIGITAL_SEPARATE => Signal::DigitalSeparate(
                unsafe { std::mem::transmute(&*(*self.0).digital_separate) }
            ),
            _ => Signal::Unknown,
        }
    }
}

pub enum Stereo {
    None,
    FieldSeqRight,
    FieldSeqLeft,
    TwoWayInterleavedRight,
    TwoWayInterleavedLeft,
    FourWayInterleaved,
    SideBySideInterleaved,
    Unknown(u32),
}

impl From<display_info_sys::di_edid_detailed_timing_def_stereo> for Stereo {
    fn from(value: display_info_sys::di_edid_detailed_timing_def_stereo) -> Self {
        match value.0 {
            0 => Stereo::None,
            1 => Stereo::FieldSeqRight,
            2 => Stereo::FieldSeqLeft,
            3 => Stereo::TwoWayInterleavedRight,
            4 => Stereo::TwoWayInterleavedLeft,
            5 => Stereo::FourWayInterleaved,
            6 => Stereo::SideBySideInterleaved,
            x => Stereo::Unknown(x),
        }
    }
}

pub enum Signal<'a> {
    AnalogComposite(&'a AnalogComposite),
    BipolarAnalogComposite(&'a BipolarAnalogComposite),
    DigitalComposite(&'a DigitalComposite),
    DigitalSeparate(&'a DigitalSeparate),
    Unknown,
}
