/*
let audioinfo = libspa_sys::spa_audio_info_raw {
		format: libspa_sys::SPA_AUDIO_FORMAT_S16  as isize,
		flags: 0,
		rate: 48000,
		channels: 2,
		position: arr!(libspa_sys::spa_audio_channel_SPA_AUDIO_CHANNEL_FL, libspa_sys::spa_audio_channel_SPA_AUDIO_CHANNEL_FR, 0;62),
	};
	*/

use libspa::pod::serialize::{GenError, PodSerialize, PodSerializer, SerializeSuccess};
use libspa::pod::PropertyFlags;
use libspa::utils::Id;
use std::io;

use arr_macro::arr;

#[allow(dead_code, non_upper_case_globals, non_snake_case)]
pub mod Format {
	pub type Format = u32;
	pub const Unknown: Format = libspa_sys::SPA_AUDIO_FORMAT_UNKNOWN;
	pub const Encoded: Format = libspa_sys::SPA_AUDIO_FORMAT_ENCODED;
	pub const StartInterleaved: Format = libspa_sys::SPA_AUDIO_FORMAT_START_Interleaved;
	pub const S8: Format = libspa_sys::SPA_AUDIO_FORMAT_S8;
	pub const U8: Format = libspa_sys::SPA_AUDIO_FORMAT_U8;
	pub const S16_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_S16_LE;
	pub const S16_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_S16_BE;
	pub const U16_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_U16_LE;
	pub const U16_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_U16_BE;
	pub const S24_32_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_32_LE;
	pub const S24_32_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_32_BE;
	pub const U24_32_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_U24_32_LE;
	pub const U24_32_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_U24_32_BE;
	pub const S32_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_S32_LE;
	pub const S32_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_S32_BE;
	pub const U32_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_U32_LE;
	pub const U32_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_U32_BE;
	pub const S24_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_LE;
	pub const S24_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_BE;
	pub const U24_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_U24_LE;
	pub const U24_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_U24_BE;
	pub const S20_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_S20_LE;
	pub const S20_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_S20_BE;
	pub const U20_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_U20_LE;
	pub const U20_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_U20_BE;
	pub const S18_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_S18_LE;
	pub const S18_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_S18_BE;
	pub const U18_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_U18_LE;
	pub const U18_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_U18_BE;
	pub const F32_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_F32_LE;
	pub const F32_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_F32_BE;
	pub const F64_LE: Format = libspa_sys::SPA_AUDIO_FORMAT_F64_LE;
	pub const F64_BE: Format = libspa_sys::SPA_AUDIO_FORMAT_F64_BE;

	pub const ULAW: Format = libspa_sys::SPA_AUDIO_FORMAT_ULAW;
	pub const ALAW: Format = libspa_sys::SPA_AUDIO_FORMAT_ALAW;

	pub const StartPlanar: Format = libspa_sys::SPA_AUDIO_FORMAT_START_Planar;

	pub const U8P: Format = libspa_sys::SPA_AUDIO_FORMAT_U8P;
	pub const S16P: Format = libspa_sys::SPA_AUDIO_FORMAT_S16P;
	pub const S24_32P: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_32P;
	pub const S32P: Format = libspa_sys::SPA_AUDIO_FORMAT_S32P;
	pub const S24P: Format = libspa_sys::SPA_AUDIO_FORMAT_S24P;
	pub const F32P: Format = libspa_sys::SPA_AUDIO_FORMAT_F32P;
	pub const F64P: Format = libspa_sys::SPA_AUDIO_FORMAT_F64P;
	pub const S8P: Format = libspa_sys::SPA_AUDIO_FORMAT_S8P;

	pub const StartOther: Format = libspa_sys::SPA_AUDIO_FORMAT_START_Other;

	pub const DSP_S32: Format = libspa_sys::SPA_AUDIO_FORMAT_DSP_S32;
	pub const DSP_F32: Format = libspa_sys::SPA_AUDIO_FORMAT_DSP_F32;
	pub const DSP_F64: Format = libspa_sys::SPA_AUDIO_FORMAT_DSP_F64;

	pub const S16: Format = libspa_sys::SPA_AUDIO_FORMAT_S16;
	pub const U16: Format = libspa_sys::SPA_AUDIO_FORMAT_U16;
	pub const S24_32: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_32;
	pub const U24_32: Format = libspa_sys::SPA_AUDIO_FORMAT_U24_32;
	pub const S32: Format = libspa_sys::SPA_AUDIO_FORMAT_S32;
	pub const U32: Format = libspa_sys::SPA_AUDIO_FORMAT_U32;
	pub const S24: Format = libspa_sys::SPA_AUDIO_FORMAT_S24;
	pub const U24: Format = libspa_sys::SPA_AUDIO_FORMAT_U24;
	pub const S20: Format = libspa_sys::SPA_AUDIO_FORMAT_S20;
	pub const U20: Format = libspa_sys::SPA_AUDIO_FORMAT_U20;
	pub const S18: Format = libspa_sys::SPA_AUDIO_FORMAT_S18;
	pub const U18: Format = libspa_sys::SPA_AUDIO_FORMAT_U18;
	pub const F32: Format = libspa_sys::SPA_AUDIO_FORMAT_F32;
	pub const F64: Format = libspa_sys::SPA_AUDIO_FORMAT_F64;

	pub const S16_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_S16_OE;
	pub const U16_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_U16_OE;
	pub const S24_32_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_32_OE;
	pub const U24_32_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_U24_32_OE;
	pub const S32_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_S32_OE;
	pub const U32_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_U32_OE;
	pub const S24_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_S24_OE;
	pub const U24_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_U24_OE;
	pub const S20_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_S20_OE;
	pub const U20_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_U20_OE;
	pub const S18_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_S18_OE;
	pub const U18_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_U18_OE;
	pub const F32_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_F32_OE;
	pub const F64_OE: Format = libspa_sys::SPA_AUDIO_FORMAT_F64_OE;
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct AudioInfo {
	pub Format: Format::Format,
	pub Rate: i32,
	pub Channels: i32,
	pub Position: [Id; 64],
}

impl AudioInfo {
	pub fn new(format: Format::Format, rate: i32, channels: i32, positions: [Id; 64]) -> Self {
		AudioInfo {
			Format: format,
			Rate: rate,
			Channels: channels,
			Position: positions,
		}
	}
	pub fn stereo(format: Format::Format, rate: i32) -> Self {
		AudioInfo::new(
			format,
			rate,
			2,
			arr!(Id(libspa_sys::SPA_AUDIO_CHANNEL_FL), Id(libspa_sys::SPA_AUDIO_CHANNEL_FR), Id(0);62),
		)
	}
}

impl PodSerialize for AudioInfo {
	fn serialize<O: io::Write + io::Seek>(
		&self,
		serializer: PodSerializer<O>,
	) -> Result<SerializeSuccess<O>, GenError> {
		let mut obj = serializer.serialize_object(
			libspa_sys::SPA_TYPE_OBJECT_Format,
			libspa_sys::SPA_PARAM_EnumFormat,
		)?;
		obj.serialize_property(
			libspa_sys::SPA_FORMAT_mediaType,
			&Id(libspa_sys::SPA_MEDIA_TYPE_audio),
			PropertyFlags::empty(),
		)?;
		obj.serialize_property(
			libspa_sys::SPA_FORMAT_mediaSubtype,
			&Id(libspa_sys::SPA_MEDIA_SUBTYPE_raw),
			PropertyFlags::empty(),
		)?;
		if self.Format != Format::Unknown {
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_format,
				&Id(self.Format),
				PropertyFlags::empty(),
			)?;
		}
		if self.Rate != 0 {
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_rate,
				&self.Rate,
				PropertyFlags::empty(),
			)?;
		}
		if self.Channels != 0 {
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_channels,
				&self.Channels,
				PropertyFlags::empty(),
			)?;
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_position,
				&self.Position[..],
				PropertyFlags::empty(),
			)?;
		}
		obj.end()
	}
}
