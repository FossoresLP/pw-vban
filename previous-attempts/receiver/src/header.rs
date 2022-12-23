struct Header {
	Protocol: Proto,
	Rate: Rate,
	Info: u8,    // Sample count for audio / bitmode for serial / zero for text
	Channel: u8, // Channel count for audio / channel ID for serial and text
	Type: StreamType,
	Format: Format,
	Name: String,
	Counter: u32, // optional, can be zero
}

enum Proto {
	Audio = 0x0,
	Serial = 0x1,
	Text = 0x2,
}

// Rate is the stream rate (Baud Rate in B/s for Serial / Text and Sample Rate in kHz for Audio)
enum Rate {
	BR0 = 0,
	BR110 = 1,
	BR150 = 2,
	BR300 = 3,
	BR600 = 4,
	BR1200 = 5,
	BR2400 = 6,
	BR4800 = 7,
	BR9600 = 8,
	BR14400 = 9,
	BR19200 = 10,
	BR31250 = 11,
	BR38400 = 12,
	BR57600 = 13,
	BR115200 = 14,
	BR128000 = 15,
	BR230400 = 16,
	BR250000 = 17,
	BR256000 = 18,
	BR460800 = 19,
	BR921600 = 20,
	BR1000000 = 21,
	BR1500000 = 22,
	BR2000000 = 23,
	BR3000000 = 24,
	SR6kHz = BR0,
	SR12kHz = BR110,
	SR24kHz = BR150,
	SR48kHz = BR300,
	SR96kHz = BR600,
	SR192kHz = BR1200,
	SR384kHz = BR2400,
	SR8kHz = BR4800,
	SR16kHz = BR9600,
	SR32kHz = BR14400,
	SR64kHz = BR19200,
	SR128kHz = BR31250,
	SR256kHz = BR38400,
	SR512kHz = BR57600,
	SR11_025kHz = BR115200,
	SR22_050kHz = BR128000,
	SR44_1kHz = BR230400,
	SR88_2kHz = BR250000,
	SR176_4kHz = BR256000,
	SR352_8kHz = BR460800,
	SR705_6kHz = BR921600,
}

// Byte Format of the data stream
enum Format {
	U8 = 0x0,  // Unsigned 8 bit / 1 byte - only option for serial and text
	S16 = 0x1, // Signed 16 bit / 2 byte
	S24 = 0x2, // Signed 24 bit / 3 byte
	S32 = 0x3, // Signed 32 bit / 4 byte
	F32 = 0x4, // Float 32 bit / 4 byte
	F64 = 0x5, // Float 64 bit / 4 byte
	S12 = 0x6, // Signed 12 bit - not supported by SPA/pipewire
	S10 = 0x7, // Signed 10 bit - not supported by SPA/pipewire
}

// High-level Data Type of the stream (audio codec, text encoding, etc)
enum StreamType {
	Undef0 = 0,
	Undef1 = 1,
	Undef2 = 2,
	Undef3 = 3,
	Undef4 = 4,
	Undef5 = 5,
	Undef6 = 6,
	Undef7 = 7,
	Undef8 = 8,
	Undef9 = 9,
	Undef10 = 10,
	Undef11 = 11,
	Undef12 = 12,
	Undef13 = 13,
	Undef14 = 14,
	User = 15,        // User defined (possible for text, serial and audio)
	ASCII = Undef0,   // ASCII Text
	UTF8 = Undef1,    // UTF-8 Text
	WCHAR = Undef2,   // UTF-16 Text
	Generic = Undef0, // Serial Generic
	MIDI = Undef1,    // Serial MIDI
	PCM = Undef0,     // PCM codec
	VBCA = Undef1,    // VB Codec Audio
	VBCV = Undef2,    // VB Codec Voice
}
