pub fn charmap() -> [&'static str, .. 256]{ return ["\x00", // 0x0
"\x01", // 0x1
"\x02", // 0x2
"\x03", // 0x3
"\x04", // 0x4
"\x05", // 0x5
"\x06", // 0x6
"\x07", // 0x7
"\x08", // 0x8
"\t", // 0x9
"\n", // 0xa
"\x0b", // 0xb
"\x0c", // 0xc
"\r", // 0xd
"\x0e", // 0xe
"\x0f", // 0xf
"\x10", // 0x10
"\x11", // 0x11
"\x12", // 0x12
"\x13", // 0x13
"\x14", // 0x14
"\x15", // 0x15
"\x16", // 0x16
"\x17", // 0x17
"\x18", // 0x18
"\x19", // 0x19
"\x1a", // 0x1a
"\x1b", // 0x1b
"\x1c", // 0x1c
"\x1d", // 0x1d
"\x1e", // 0x1e
"\x1f", // 0x1f
" ", // 0x20
"!", // 0x21
"\"", // 0x22
"#", // 0x23
"$", // 0x24
"%", // 0x25
"&", // 0x26
"'", // 0x27
"(", // 0x28
")", // 0x29
"*", // 0x2a
"+", // 0x2b
",", // 0x2c
"-", // 0x2d
".", // 0x2e
"/", // 0x2f
"0", // 0x30
"1", // 0x31
"2", // 0x32
"3", // 0x33
"4", // 0x34
"5", // 0x35
"6", // 0x36
"7", // 0x37
"8", // 0x38
"9", // 0x39
":", // 0x3a
";", // 0x3b
"<", // 0x3c
"=", // 0x3d
">", // 0x3e
"?", // 0x3f
"@", // 0x40
"A", // 0x41
"B", // 0x42
"C", // 0x43
"D", // 0x44
"E", // 0x45
"F", // 0x46
"G", // 0x47
"H", // 0x48
"I", // 0x49
"J", // 0x4a
"K", // 0x4b
"L", // 0x4c
"M", // 0x4d
"N", // 0x4e
"O", // 0x4f
"P", // 0x50
"Q", // 0x51
"R", // 0x52
"S", // 0x53
"T", // 0x54
"U", // 0x55
"V", // 0x56
"W", // 0x57
"X", // 0x58
"Y", // 0x59
"Z", // 0x5a
"[", // 0x5b
"\\", // 0x5c
"]", // 0x5d
"^", // 0x5e
"_", // 0x5f
"`", // 0x60
"a", // 0x61
"b", // 0x62
"c", // 0x63
"d", // 0x64
"e", // 0x65
"f", // 0x66
"g", // 0x67
"h", // 0x68
"i", // 0x69
"j", // 0x6a
"k", // 0x6b
"l", // 0x6c
"m", // 0x6d
"n", // 0x6e
"o", // 0x6f
"p", // 0x70
"q", // 0x71
"r", // 0x72
"s", // 0x73
"t", // 0x74
"u", // 0x75
"v", // 0x76
"w", // 0x77
"x", // 0x78
"y", // 0x79
"z", // 0x7a
"{", // 0x7b
"|", // 0x7c
"}", // 0x7d
"~", // 0x7e
"\x7f", // 0x7f
"\x80", // 0x80
"\x81", // 0x81
"\x82", // 0x82
"\x83", // 0x83
"\x84", // 0x84
"\x85", // 0x85
"\x86", // 0x86
"\x87", // 0x87
"\x88", // 0x88
"\x89", // 0x89
"\x8a", // 0x8a
"\x8b", // 0x8b
"\x8c", // 0x8c
"\x8d", // 0x8d
"\x8e", // 0x8e
"\x8f", // 0x8f
"\x90", // 0x90
"\x91", // 0x91
"\x92", // 0x92
"\x93", // 0x93
"\x94", // 0x94
"\x95", // 0x95
"\x96", // 0x96
"\x97", // 0x97
"\x98", // 0x98
"\x99", // 0x99
"\x9a", // 0x9a
"\x9b", // 0x9b
"\x9c", // 0x9c
"\x9d", // 0x9d
"\x9e", // 0x9e
"\x9f", // 0x9f
"\xa0", // 0xa0
"\u0104", // 0xa1
"\x2d8", // 0xa2
"\u0141", // 0xa3
"\xa4", // 0xa4
"\u013d", // 0xa5
"\u015a", // 0xa6
"\xa7", // 0xa7
"\xa8", // 0xa8
"\u0160", // 0xa9
"\u015e", // 0xaa
"\u0164", // 0xab
"\u0179", // 0xac
"\xad", // 0xad
"\u017d", // 0xae
"\u017b", // 0xaf
"\xb0", // 0xb0
"\u0105", // 0xb1
"\x2db", // 0xb2
"\u0142", // 0xb3
"\xb4", // 0xb4
"\u013e", // 0xb5
"\u015b", // 0xb6
"\x2c7", // 0xb7
"\xb8", // 0xb8
"\u0161", // 0xb9
"\u015f", // 0xba
"\u0165", // 0xbb
"\u017a", // 0xbc
"\x2dd", // 0xbd
"\u017e", // 0xbe
"\u017c", // 0xbf
"\u0154", // 0xc0
"\xc1", // 0xc1
"\xc2", // 0xc2
"\u0102", // 0xc3
"\xc4", // 0xc4
"\u0139", // 0xc5
"\u0106", // 0xc6
"\xc7", // 0xc7
"\u010c", // 0xc8
"\xc9", // 0xc9
"\u0118", // 0xca
"\xcb", // 0xcb
"\u011a", // 0xcc
"\xcd", // 0xcd
"\xce", // 0xce
"\u010e", // 0xcf
"\u0110", // 0xd0
"\u0143", // 0xd1
"\u0147", // 0xd2
"\xd3", // 0xd3
"\xd4", // 0xd4
"\u0150", // 0xd5
"\xd6", // 0xd6
"\xd7", // 0xd7
"\u0158", // 0xd8
"\u016e", // 0xd9
"\xda", // 0xda
"\u0170", // 0xdb
"\xdc", // 0xdc
"\xdd", // 0xdd
"\u0162", // 0xde
"\xdf", // 0xdf
"\u0155", // 0xe0
"\xe1", // 0xe1
"\xe2", // 0xe2
"\u0103", // 0xe3
"\xe4", // 0xe4
"\u013a", // 0xe5
"\u0107", // 0xe6
"\xe7", // 0xe7
"\u010d", // 0xe8
"\xe9", // 0xe9
"\u0119", // 0xea
"\xeb", // 0xeb
"\u011b", // 0xec
"\xed", // 0xed
"\xee", // 0xee
"\u010f", // 0xef
"\u0111", // 0xf0
"\u0144", // 0xf1
"\u0148", // 0xf2
"\xf3", // 0xf3
"\xf4", // 0xf4
"\u0151", // 0xf5
"\xf6", // 0xf6
"\xf7", // 0xf7
"\u0159", // 0xf8
"\u016f", // 0xf9
"\xfa", // 0xfa
"\u0171", // 0xfb
"\xfc", // 0xfc
"\xfd", // 0xfd
"\u0163", // 0xfe
"\x2d9", // 0xff
];}