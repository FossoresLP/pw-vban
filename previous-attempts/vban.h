#pragma once

#include <stdint.h>

struct VBAN_HEADER {
	uint32_t VBAN; /* contains 'V' 'B', 'A', 'N' */
	uint8_t SampleRate;
	uint8_t Samples;
	uint8_t Channels;
	uint8_t Format;
	char Name[16];
	uint32_t Counter;
} __attribute__((packed));

enum SampleRate {
	SR_6kHz,
	SR_12kHz,
	SR_24kHz,
	SR_48kHz,
	SR_96kHz,
	SR_192kHz,
	SR_384kHz,
	SR_8kHz,
	SR_16kHz,
	SR_32kHz,
	SR_64kHz,
	SR_128kHz,
	SR_256kHz,
	SR_512kHz,
	SR_11_025kHz,
	SR_22_050kHz,
	SR_44_1kHz,
	SR_88_2kHz,
	SR_176_4kHz,
	SR_352_8kHz,
	SR_705_6kHz
};

enum Format {
	U8,	  // 0x00
	S16,  // 0x01
	S24,  // 0x02
	S32,  // 0x03
	F32,  // 0x04
	F64,  // 0x05
		  // 12b 0x06 - not supported by SPA/pipewire
		  // 10b 0x07 - not supported by SPA/pipewire
};
