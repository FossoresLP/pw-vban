use libspa::pod::serialize::{GenError, PodSerialize, PodSerializer, SerializeSuccess};
use libspa::pod::PropertyFlags;
use libspa::utils::Id;
use std::io;
use std::io::Cursor;

use arr_macro::arr;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Format {
	Unknown = libspa_sys::SPA_AUDIO_FORMAT_UNKNOWN,
	Encoded = libspa_sys::SPA_AUDIO_FORMAT_ENCODED,

	// Interleaved
	S8 = libspa_sys::SPA_AUDIO_FORMAT_S8,
	U8 = libspa_sys::SPA_AUDIO_FORMAT_U8,
	S16LE = libspa_sys::SPA_AUDIO_FORMAT_S16_LE,
	S16BE = libspa_sys::SPA_AUDIO_FORMAT_S16_BE,
	U16LE = libspa_sys::SPA_AUDIO_FORMAT_U16_LE,
	U16BE = libspa_sys::SPA_AUDIO_FORMAT_U16_BE,
	S24in32LE = libspa_sys::SPA_AUDIO_FORMAT_S24_32_LE,
	S24in32BE = libspa_sys::SPA_AUDIO_FORMAT_S24_32_BE,
	U24in32LE = libspa_sys::SPA_AUDIO_FORMAT_U24_32_LE,
	U24in32BE = libspa_sys::SPA_AUDIO_FORMAT_U24_32_BE,
	S32LE = libspa_sys::SPA_AUDIO_FORMAT_S32_LE,
	S32BE = libspa_sys::SPA_AUDIO_FORMAT_S32_BE,
	U32LE = libspa_sys::SPA_AUDIO_FORMAT_U32_LE,
	U32BE = libspa_sys::SPA_AUDIO_FORMAT_U32_BE,
	S24LE = libspa_sys::SPA_AUDIO_FORMAT_S24_LE,
	S24BE = libspa_sys::SPA_AUDIO_FORMAT_S24_BE,
	U24LE = libspa_sys::SPA_AUDIO_FORMAT_U24_LE,
	U24BE = libspa_sys::SPA_AUDIO_FORMAT_U24_BE,
	S20LE = libspa_sys::SPA_AUDIO_FORMAT_S20_LE,
	S20BE = libspa_sys::SPA_AUDIO_FORMAT_S20_BE,
	U20LE = libspa_sys::SPA_AUDIO_FORMAT_U20_LE,
	U20BE = libspa_sys::SPA_AUDIO_FORMAT_U20_BE,
	S18LE = libspa_sys::SPA_AUDIO_FORMAT_S18_LE,
	S18BE = libspa_sys::SPA_AUDIO_FORMAT_S18_BE,
	U18LE = libspa_sys::SPA_AUDIO_FORMAT_U18_LE,
	U18BE = libspa_sys::SPA_AUDIO_FORMAT_U18_BE,
	F32LE = libspa_sys::SPA_AUDIO_FORMAT_F32_LE,
	F32BE = libspa_sys::SPA_AUDIO_FORMAT_F32_BE,
	F64LE = libspa_sys::SPA_AUDIO_FORMAT_F64_LE,
	F64BE = libspa_sys::SPA_AUDIO_FORMAT_F64_BE,

	// Codecs
	ULAW = libspa_sys::SPA_AUDIO_FORMAT_ULAW,
	ALAW = libspa_sys::SPA_AUDIO_FORMAT_ALAW,

	// Planar
	U8P = libspa_sys::SPA_AUDIO_FORMAT_U8P,
	S16P = libspa_sys::SPA_AUDIO_FORMAT_S16P,
	S24in32P = libspa_sys::SPA_AUDIO_FORMAT_S24_32P,
	S32P = libspa_sys::SPA_AUDIO_FORMAT_S32P,
	S24P = libspa_sys::SPA_AUDIO_FORMAT_S24P,
	F32P = libspa_sys::SPA_AUDIO_FORMAT_F32P,
	F64P = libspa_sys::SPA_AUDIO_FORMAT_F64P,
	S8P = libspa_sys::SPA_AUDIO_FORMAT_S8P,
}

#[derive(Debug)]
pub struct AudioInfo {
	pub format: Format,
	pub rate: i32,
	pub channels: i32,
	pub position: [Id; 64],
}

impl AudioInfo {
	pub fn new(format: Format, rate: i32, channels: i32, positions: [Id; 64]) -> Self {
		AudioInfo {
			format,
			rate,
			channels,
			position: positions,
		}
	}
	pub fn stereo(format: Format, rate: i32) -> Self {
		AudioInfo::new(
			format,
			rate,
			2,
			arr!(Id(libspa_sys::SPA_AUDIO_CHANNEL_FL), Id(libspa_sys::SPA_AUDIO_CHANNEL_FR), Id(0);62),
		)
	}
	pub fn build(self: &Self) -> Result<*const libspa_sys::spa_pod, GenError> {
		let pod: Cursor<Vec<u8>> = Cursor::new(Vec::new());

		let pod = libspa::pod::serialize::PodSerializer::serialize(pod, self)?.0;

		Ok(Box::into_raw(pod.into_inner().into_boxed_slice()) as *const libspa_sys::spa_pod)
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
		if self.format != Format::Unknown {
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_format,
				&Id(self.format as u32),
				PropertyFlags::empty(),
			)?;
		}
		if self.rate != 0 {
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_rate,
				&self.rate,
				PropertyFlags::empty(),
			)?;
		}
		if self.channels != 0 {
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_channels,
				&self.channels,
				PropertyFlags::empty(),
			)?;
			obj.serialize_property(
				libspa_sys::SPA_FORMAT_AUDIO_position,
				&self.position[..],
				PropertyFlags::empty(),
			)?;
		}
		obj.end()
	}
}
