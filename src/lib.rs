use std::io::{Read,Write};
use std::ascii::AsciiExt;

#[derive(Debug)]
struct Bom ( u8, u8, u8, u8 );

#[derive(Clone,Debug)]
enum Flavour {
    UCS,
    UTF,
    EBCDIC,
    ASCII,
    Unknown,
}

#[derive(Clone,Debug)]
enum ByteOrder {
    BigEndian,
    LittleEndian,
    Unusual2143,
    Unusual3412,
    NotApplicable
}

#[derive(Clone,Debug)]
enum Width {
    EightBit     = 8,
    SixteenBit   = 16,
    ThirtyTwoBit = 32
}

#[derive(Clone,Debug)]
struct Descriptor (
    Flavour,
    Width,
    ByteOrder,
);

// 32-Bit Encodings
const UCS_4_BE: Descriptor = Descriptor(Flavour::UCS, Width::ThirtyTwoBit, ByteOrder::BigEndian);
const UCS_4_LE: Descriptor = Descriptor(Flavour::UCS, Width::ThirtyTwoBit, ByteOrder::LittleEndian);
const UCS_4_2143: Descriptor = Descriptor(Flavour::UCS, Width::ThirtyTwoBit, ByteOrder::Unusual2143);
const UCS_4_3412: Descriptor = Descriptor(Flavour::UCS, Width::ThirtyTwoBit, ByteOrder::Unusual3412);

// 16-Bit Encodings
const UTF_16_BE: Descriptor = Descriptor(Flavour::UTF, Width::SixteenBit, ByteOrder::BigEndian);
const UTF_16_LE: Descriptor = Descriptor(Flavour::UTF, Width::SixteenBit, ByteOrder::LittleEndian);

const UTF_8: Descriptor = Descriptor(Flavour::UTF, Width::EightBit, ByteOrder::NotApplicable);
const EBCDIC: Descriptor = Descriptor(Flavour::EBCDIC, Width::EightBit, ByteOrder::NotApplicable);

// ASCII compatible encodings
const ASCII_32BIT_BE: Descriptor = Descriptor(Flavour::Unknown, Width::ThirtyTwoBit, ByteOrder::BigEndian);
const ASCII_32BIT_LE: Descriptor = Descriptor(Flavour::Unknown, Width::ThirtyTwoBit, ByteOrder::LittleEndian);
const ASCII_16BIT_BE: Descriptor = Descriptor(Flavour::Unknown, Width::SixteenBit, ByteOrder::BigEndian);
const ASCII_16BIT_LE: Descriptor = Descriptor(Flavour::Unknown, Width::SixteenBit, ByteOrder::LittleEndian);
const ASCII_8BIT: Descriptor = Descriptor(Flavour::ASCII, Width::EightBit, ByteOrder::NotApplicable);

pub fn detect<R: Read>(reader: &mut R, hint: Option<String>) -> Vec<String> {
    // Read the first 4 bytes and see if they help
    let mut first_four_bytes = [0u8; 4];
    match reader.read(&mut first_four_bytes) {
        Ok(bytes_read) => assert_eq!(bytes_read, 4),
        Err(e) => panic!(e)
    };

    let bom = Bom(
        first_four_bytes[0],
        first_four_bytes[1],
        first_four_bytes[2],
        first_four_bytes[3]
    );

    // http://www.w3.org/TR/2004/REC-xml-20040204/#sec-guessing-no-ext-info
    // Can do below without the Bom type if slice pattern syntax becomes non-experimental
    // let possible = match first_four_bytes {
    //     // With Byte Order Mark
    //     [0x00, 0x00, 0xFE, 0xFF]             => Some("UCS-4BE"),
    //     [0xFF, 0xFE, 0x00, 0x00]             => Some("UCS-4LE"),
    //     [0x00, 0x00, 0xFF, 0xFE]             => Some("UCS-4OE"),
    //     [0xFE, 0xFF, 0x00, 0x00]             => Some("UCS-4EO"),
    //     [0xFE, 0xFF, c, d] if c > 0 && d > 0 => Some("UTF-16BE"),
    //     [0xFF, 0xFE, c, d] if c > 0 && d > 0 => Some("UCS-16LE"),
    //     [0xEF, 0xBB, 0xBF, _   ]             => Some("UTF-8"),

    //     //  Without Byte Order Mark
    //     [0x00, 0x00, 0x00, 0x3C] |
    //     [0x3C, 0x00, 0x00, 0x00] |
    //     [0x00, 0x00, 0x3C, 0x00] |
    //     [0x00, 0x3C, 0x00, 0x00]             => Some("32-bit"),
    //     [0x00, 0x3C, 0x00, 0x3F]             => Some("16-bit Big Endian"),
    //     [0x3C, 0x00, 0x3F, 0x00]             => Some("16-bit Little Endian"),
    //     [0x3C, 0x3F, 0x78, 0x6D]             => Some("8-bit"),
    //     [0x4C, 0x6F, 0xA7, 0x94]             => Some("EBCDIC"),
    //     // This may be UTF-8 without an encoding declaration as this is not required
    //     // for UTF-8
    //     _                                    => Some("Other"),
    // };

    let possible = match bom {
        // With Byte Order Mark
        Bom(0x00, 0x00, 0xFE, 0xFF)                   => Some(UCS_4_BE),
        Bom(0xFF, 0xFE, 0x00, 0x00)                   => Some(UCS_4_LE),
        Bom(0x00, 0x00, 0xFF, 0xFE)                   => Some(UCS_4_2143),
        Bom(0xFE, 0xFF, 0x00, 0x00)                   => Some(UCS_4_3412),
        Bom(0xFE, 0xFF, c   , d   ) if c > 0 || d > 0 => Some(UTF_16_BE),
        Bom(0xFF, 0xFE, c   , d   ) if c > 0 || d > 0 => Some(UTF_16_LE),
        Bom(0xEF, 0xBB, 0xBF, _   )                   => Some(UTF_8),

        //  Without Byte Order Mark
        Bom(0x00, 0x00, 0x00, 0x3C)                   => Some(ASCII_32BIT_BE),
        Bom(0x3C, 0x00, 0x00, 0x00)                   => Some(ASCII_32BIT_LE),
        Bom(0x00, 0x00, 0x3C, 0x00)                   => Some(Descriptor(Flavour::Unknown, Width::ThirtyTwoBit, ByteOrder::Unusual2143)),
        Bom(0x00, 0x3C, 0x00, 0x00)                   => Some(Descriptor(Flavour::Unknown, Width::ThirtyTwoBit, ByteOrder::Unusual3412)),
        Bom(0x00, 0x3C, 0x00, 0x3F)                   => Some(ASCII_16BIT_BE),
        Bom(0x3C, 0x00, 0x3F, 0x00)                   => Some(ASCII_16BIT_LE),
        Bom(0x3C, 0x3F, 0x78, 0x6D)                   => Some(ASCII_8BIT),
        Bom(0x4C, 0x6F, 0xA7, 0x94)                   => Some(EBCDIC),
        // This may be UTF-8 without an encoding declaration as this is not required
        // for UTF-8
        _                                             => None,
    };

    // Now that byte size may have been determined try reading the first 512ish bytes to read an
    // encoding declaration
    let mut buf = [0u8; 512];
    let bytes_read = reader.read(&mut buf).unwrap();

    let mut candidates = Vec::with_capacity(3);

    // Look for encoding="", charset="?"?
    search("encoding=", &buf.to_vec(), possible.clone())
        .or_else(|| search("charset=", &buf.to_vec(), possible.clone()))
        .map(|encoding| normalise(&encoding))
        .map(|encoding| push_if_not_contains(&mut candidates, endianify(&encoding, possible.clone())));

    // Consider hint
    hint.map(|hint| normalise(&hint))
        .map(|encoding| push_if_not_contains(&mut candidates, endianify(&encoding, possible.clone())));

    // Include info from BOM detection
    let from_bom = match possible {
        Some(UCS_4_LE) => Some("ucs-4le"),
        Some(UCS_4_BE) => Some("ucs-4be"),
        Some(UTF_16_LE) => Some("utf-16le"),
        Some(UTF_16_BE) => Some("utf-16be"),
        Some(Descriptor(Flavour::UTF, Width::EightBit, _)) => Some("utf-8"),
        Some(EBCDIC) => Some("ebcdic"),
        _ => None
    }.map(|encoding| push_if_not_contains(&mut candidates, encoding.to_string()));

    // Otherwise test if UTF-8
    if candidates.is_empty() && String::from_utf8(buf.to_vec()).is_ok() {
        candidates.push("utf-8".to_string());
    }

    return candidates;
}

fn normalise(encoding: &String) -> String {
    encoding.to_ascii_lowercase()
        .replace("us-ascii", "ascii")
        .replace("shift-jis", "shift_jis")
}

fn push_if_not_contains<T: PartialEq>(vec: &mut Vec<T>, item: T) {
    if !vec.contains(&item) {
        vec.push(item);
    }
}

fn endianify(encoding: &str, descriptor: Option<Descriptor>) -> String {
    let Descriptor(ref flavour, ref width, ref order) = descriptor.unwrap_or(ASCII_8BIT);

    match encoding {
        "utf-16" => {
            match *order {
                ByteOrder::LittleEndian => "utf-16le".to_string(),
                ByteOrder::BigEndian    => "utf-16be".to_string(),
                _ => encoding.to_string()
            }
        },
        _ => encoding.to_string()
    }
}

fn search(needle: &str, haystack: &Vec<u8>, descriptor: Option<Descriptor>) -> Option<String> {
    let Descriptor(ref flavour, ref width, ref order) = descriptor.unwrap_or(ASCII_8BIT);
    let chunk_size = (width.clone() as usize) / 8;
    let needle_bytes = needle.as_bytes();

    let mut index = match *order {
        ByteOrder::NotApplicable | ByteOrder::LittleEndian  => 0,
        ByteOrder::BigEndian => chunk_size - 1,
        ByteOrder::Unusual2143 => 2,
        ByteOrder::Unusual3412 => 1,
    };

    let mut ascii_bytes = Vec::with_capacity(haystack.len() / chunk_size);
    while index < haystack.len() {
        ascii_bytes.push(haystack[index]);
        index += chunk_size;
    }

    let ascii_haystack = String::from_utf8_lossy(&ascii_bytes);

    ascii_haystack.find(needle).map(|pos| {
        // Skip to the matching byte + length of the needle
        ascii_haystack[pos + needle.len()..].chars()
            .skip_while(|char| *char == '"' || *char == '\'')
            .take_while(|char| *char != '"' && *char != '\'').collect()
    })
}

