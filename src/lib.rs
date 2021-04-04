#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use core::fmt::Display;
extern crate unicode_normalization;
use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;


//#[macro_use]
//extern crate bitflags;

pub const HGK_NO_DIACRITICS :u32 = 0x000;
pub const HGK_ROUGH         :u32 = 0x001;
pub const HGK_SMOOTH        :u32 = 0x002;
pub const HGK_ACUTE         :u32 = 0x004;
pub const HGK_GRAVE         :u32 = 0x008;
pub const HGK_CIRCUMFLEX    :u32 = 0x010;
pub const HGK_MACRON        :u32 = 0x020;
pub const HGK_BREVE         :u32 = 0x040;
pub const HGK_IOTA_SUBSCRIPT:u32 = 0x080;
pub const HGK_DIAERESIS     :u32 = 0x100;
pub const HGK_UNDERDOT      :u32 = 0x200;
/*
bitflags! {
    pub struct HGKDiacritics: u32 {
        const ROUGH          = 0x001;
        const SMOOTH         = 0x002;
        const ACUTE          = 0x004;
        const GRAVE          = 0x008;
        const CIRCUMFLEX     = 0x010;
        const MACRON         = 0x020;
        const BREVE          = 0x040;
        const IOTA_SUBSCRIPT = 0x080;
        const DIAERESIS      = 0x100;
        const UNDERDOT       = 0x200;
        //const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}
*/
pub enum HGKUnicode_Mode {
    Precomposed,
    PrecomposedPUA,
    CombiningOnly
}

struct HGKLetter {
    letter: char,
    //diacritics: HGKDiacritics
    diacritics: u32
}

impl HGKLetter {
    fn from_str(l:&str) -> HGKLetter {
        //let mut diacritics: HGKDiacritics = HGKDiacritics::empty();
        let mut diacritics:u32 = 0;
        let mut bare_letter: char = '\u{0000}';
        for (i, c) in l.nfd().enumerate() {
            if i == 0 {
                assert_eq!(unicode_normalization::char::is_combining_mark(c), false); //"First char of letter is a combining mark.");
                
                if c as u32 >= 0xEAF0 && c as u32 <= 0xEB8A {
                    bare_letter = GREEK_PUA[c as usize - 0xEAF0].0;
                    diacritics = GREEK_PUA[c as usize - 0xEAF0].1;
                }
                else {
                   bare_letter = c;
                }
            }
            else {
                if !unicode_normalization::char::is_combining_mark(c) {
                    break;
                }
                else {
                    match c {
                        '\u{0300}' => diacritics |= HGK_GRAVE, //HGKDiacritics::GRAVE,
                        '\u{0301}' => diacritics |= HGK_ACUTE, //HGKDiacritics::ACUTE,
                        '\u{0304}' => diacritics |= HGK_MACRON, //HGKDiacritics::MACRON,
                        '\u{0306}' => diacritics |= HGK_BREVE, //HGKDiacritics::BREVE,
                        '\u{0308}' => diacritics |= HGK_DIAERESIS, //HGKDiacritics::DIAERESIS,
                        '\u{0313}' => diacritics |= HGK_SMOOTH, //HGKDiacritics::SMOOTH,
                        '\u{0314}' => diacritics |= HGK_ROUGH, //HGKDiacritics::ROUGH,
                        '\u{0323}' => diacritics |= HGK_UNDERDOT, //HGKDiacritics::UNDERDOT,
                        '\u{0342}' => diacritics |= HGK_CIRCUMFLEX, //HGKDiacritics::CIRCUMFLEX,
                        '\u{0345}' => diacritics |= HGK_IOTA_SUBSCRIPT, //HGKDiacritics::IOTA_SUBSCRIPT,
                        _ => ()
                    }
                }
            }
        }
        return HGKLetter { letter: bare_letter, diacritics: diacritics };
    }
/*
COMBINING_MACRON, 
COMBINING_BREVE, 
COMBINING_DIAERESIS, 
COMBINING_ROUGH_BREATHING, 
COMBINING_SMOOTH_BREATHING, 
COMBINING_ACUTE, 
COMBINING_GRAVE, 
COMBINING_CIRCUMFLEX, 
COMBINING_IOTA_SUBSCRIPT, 
COMBINING_UNDERDOT
*/
    fn to_string(&mut self, unicode_mode:HGKUnicode_Mode) -> String {
        //let mut s = self.letter.to_string();
        let mut s = vec![self.letter];
        if (self.diacritics & HGK_MACRON) == HGK_MACRON {
            //s = s + "\u{0304}";
            s.push('\u{0304}');
        }
        if (self.diacritics & HGK_BREVE) == HGK_BREVE {
            //s = s + "\u{0306}";
            s.push('\u{0306}');
        }
        if (self.diacritics & HGK_DIAERESIS) == HGK_DIAERESIS {
            //s = s + "\u{0308}";
            s.push('\u{0308}');
        }
        if (self.diacritics & HGK_ROUGH) == HGK_ROUGH {
            //s = s + "\u{0314}";
            s.push('\u{0314}');
        }
        if (self.diacritics & HGK_SMOOTH) == HGK_SMOOTH {
            //s = s + "\u{0313}";
            s.push('\u{0313}');
        }    
        if (self.diacritics & HGK_ACUTE) == HGK_ACUTE {
            //s = s + "\u{0301}";
            s.push('\u{0301}');
        }
        if (self.diacritics & HGK_GRAVE) == HGK_GRAVE {
            //s = s + "\u{0300}";
            s.push('\u{0300}');
        }
        if (self.diacritics & HGK_CIRCUMFLEX) == HGK_CIRCUMFLEX {
            //s = s + "\u{0342}";
            s.push('\u{0342}');
        }
        if (self.diacritics & HGK_IOTA_SUBSCRIPT) == HGK_IOTA_SUBSCRIPT {
            //s = s + "\u{0345}";
            s.push('\u{0345}');
        }
        if (self.diacritics & HGK_UNDERDOT) == HGK_UNDERDOT {
            //s = s + "\u{0323}";
            s.push('\u{0323}');
        }
        match unicode_mode {
            HGKUnicode_Mode::CombiningOnly => return s.into_iter().collect::<String>(),
            HGKUnicode_Mode::PrecomposedPUA => return s.into_iter().nfc().collect::<String>(),
            _ => return s.into_iter().nfc().collect::<String>()
        }  
    }

    fn toggle_diacritic(&mut self, d:u32, on_only:bool) {
        if !self.is_legal(d) {
            return;
        }

        if self.diacritics & d != d || on_only {
            self.diacritics |= d;
        }
        else {
            self.diacritics &= !d; //turn off: rust uses !, c uses ~
            //return;
        }
        match d {
            HGK_ROUGH => {
                self.diacritics &= !(HGK_SMOOTH | HGK_DIAERESIS);
            },
            HGK_SMOOTH => {
                self.diacritics &= !(HGK_ROUGH | HGK_DIAERESIS);
            },
            HGK_ACUTE => {
                self.diacritics &= !(HGK_GRAVE | HGK_CIRCUMFLEX);
            },
            HGK_GRAVE => {
                self.diacritics &= !(HGK_ACUTE | HGK_CIRCUMFLEX);
            },
            HGK_CIRCUMFLEX => {
                self.diacritics &= !(HGK_ACUTE | HGK_GRAVE);
            },
            HGK_MACRON => {
                self.diacritics &= !(HGK_BREVE | HGK_CIRCUMFLEX);
            },
            HGK_BREVE => {
                self.diacritics &= !(HGK_MACRON | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT);
            },
            HGK_IOTA_SUBSCRIPT => {
                self.diacritics &= !(HGK_BREVE);
            },
            HGK_DIAERESIS => {
                self.diacritics &= !(HGK_ROUGH | HGK_SMOOTH);
            },
            //HGKDiacritics::UNDERDOT => { },
            _ => {
                assert!(false, "Unknown Diacritic passed")
            }
        }
    }

    fn is_legal(&mut self, d:u32) -> bool {
        match d {
            HGK_ROUGH => {
                self.letter.is_greek_vowel() || self.letter == 'ρ' || self.letter == 'Ρ'
            },
            HGK_SMOOTH => {
                self.letter.is_greek_vowel() || self.letter == 'ρ' || self.letter == 'Ρ'
            },
            HGK_ACUTE => {
                self.letter.is_greek_vowel()
            },
            HGK_GRAVE => {
                self.letter.is_greek_vowel()
            },
            HGK_CIRCUMFLEX => {
                self.letter.is_long_or_short() | self.letter.is_long()
            },
            HGK_MACRON => {
                self.letter.is_long_or_short()
            },
            HGK_BREVE => {
                self.letter.is_long_or_short()    
            },
            HGK_IOTA_SUBSCRIPT => {
                match self.letter {
                    'α' => true,
                    'ω' => true,
                    'η' => true,
                    _ => false
                } 
            },
            HGK_DIAERESIS => {
                match self.letter {
                    'ι' => true,
                    'υ' => true,
                    _ => false
                }                
            },
            HGK_UNDERDOT => { 
                true
            },
            _ => false
        }
    }
}
/*
//https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html
impl Display for HGKLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::result::Result {
        write!(f, "{}", self.to_string2(HGKUnicode_Mode::Precomposed));
    }
}
*/
trait HGKIsLong {
    fn is_long(&self) -> bool;
}

impl HGKIsLong for char {
    fn is_long(&self) -> bool {
        match self {
            'η' => true,
            'ω' => true,
            _ => false
        }
    }
}

trait HGKIsShort {
    fn is_short(&self) -> bool;
}

impl HGKIsShort for char {
    fn is_short(&self) -> bool {
        match self {
            'ε' => true,
            'ο' => true,
            _ => false
        }
    }
}

trait HGKIsLongOrShort {
    fn is_long_or_short(&self) -> bool;
}

impl HGKIsLongOrShort for char {
    fn is_long_or_short(&self) -> bool {
        match self {
            'α' => true,
            'ι' => true,
            'υ' => true,
            _ => false
        }
    }
}

trait HGKIsGreekVowel {
    fn is_greek_vowel(&self) -> bool;
}

impl HGKIsGreekVowel for char {
    fn is_greek_vowel(&self) -> bool {
        //let letter2 = self.to_lowercase();
        match self {
            'α' => true,
            'ε' => true,
            'η' => true,
            'ι' => true,
            'ο' => true,
            'υ' => true,
            'ω' => true,
            'Α' => true,
            'Ε' => true,
            'Η' => true,
            'Ι' => true,
            'Ο' => true,
            'Υ' => true,
            'Ω' => true,
            _ => false
        }
    }
}

pub fn toggle_diacritic_str(l:&str, d:u32, on_only:bool, mode:HGKUnicode_Mode) -> String {
    let mut letter = HGKLetter::from_str(l);
    letter.toggle_diacritic(d, on_only);
    return letter.to_string(mode);
}

/*
//const UCS2 puaGreekLookUp[][2] = {
static GREEK_PUA: &'static [(char, HGKDiacritics)] = &[
    ('\u{03B1}', HGKDiacritics::MACRON )
];
*/
//pub(crate) const COMPOSITION_TABLE_KV: &[(u32, char)] = &[
static GREEK_PUA: &'static [(char, u32)] = &[
    /* EAF0 */ ( '\u{03B1}', HGK_MACRON | HGK_GRAVE ),
    /* EAF1 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EAF2 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EAF3 */ ( '\u{03B1}', HGK_MACRON | HGK_SMOOTH | HGK_GRAVE ),
    /* EAF4 */ ( '\u{03B1}', HGK_MACRON | HGK_ROUGH | HGK_GRAVE ),
    /* EAF5 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EAF6 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EAF7 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EAF8 */ ( '\u{03B1}', HGK_BREVE | HGK_GRAVE ),
    /* EAF9 */ ( '\u{03B1}', HGK_BREVE | HGK_SMOOTH ),
    /* EAFA */ ( '\u{03B1}', HGK_BREVE | HGK_SMOOTH | HGK_GRAVE ),
    /* EAFB */ ( '\u{03B1}', HGK_BREVE | HGK_ROUGH | HGK_ACUTE ),
    /* EAFC */ ( '\u{03B1}', HGK_BREVE | HGK_ROUGH | HGK_GRAVE ),
    /* EAFD */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EAFE */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EAFF */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB00 */ ( '\u{03B1}', HGK_MACRON | HGK_ACUTE ),
    /* EB01 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB02 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB03 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB04 */ ( '\u{03B1}', HGK_MACRON | HGK_SMOOTH ),
    /* EB05 */ ( '\u{03B1}', HGK_MACRON | HGK_ROUGH ),
    /* EB06 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB07 */ ( '\u{03B1}', HGK_MACRON | HGK_SMOOTH | HGK_ACUTE ),
    /* EB08 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB09 */ ( '\u{03B1}', HGK_MACRON | HGK_ROUGH | HGK_ACUTE ),
    /* EB0A */ ( '\u{03B1}', HGK_BREVE | HGK_ACUTE ),
    /* EB0B */ ( '\u{03B1}', HGK_BREVE | HGK_ROUGH ),
    /* EB0C */ ( '\u{03B1}', HGK_BREVE | HGK_SMOOTH | HGK_ACUTE ),
    /* EB0D */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB0E */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB0F */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB10 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB11 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB12 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB13 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB14 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB15 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB16 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB17 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB18 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB19 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB1A */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB1B */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB1C */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB1D */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB1E */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB1F */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB20 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB21 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB22 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB23 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB24 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB25 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB26 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB27 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB28 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB29 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB2A */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB2B */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB2C */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB2D */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB2E */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB2F */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB30 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB31 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB32 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB33 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB34 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB35 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB36 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB37 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB38 */ ( '\u{03B9}', HGK_MACRON | HGK_GRAVE ),
    /* EB39 */ ( '\u{03B9}', HGK_MACRON | HGK_ACUTE ),
    /* EB3A */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB3B */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB3C */ ( '\u{03B9}', HGK_MACRON | HGK_SMOOTH ),
    /* EB3D */ ( '\u{03B9}', HGK_MACRON | HGK_SMOOTH | HGK_ACUTE ),
    /* EB3E */ ( '\u{03B9}', HGK_MACRON | HGK_ROUGH ),
    /* EB3F */ ( '\u{03B9}', HGK_MACRON | HGK_ROUGH | HGK_ACUTE ),
    /* EB40 */ ( '\u{03B9}', HGK_BREVE | HGK_ACUTE ),
    /* EB41 */ ( '\u{03B9}', HGK_BREVE | HGK_SMOOTH ),
    /* EB42 */ ( '\u{03B9}', HGK_BREVE | HGK_SMOOTH | HGK_ACUTE ),
    /* EB43 */ ( '\u{03B9}', HGK_BREVE | HGK_ROUGH ),
    /* EB44 */ ( '\u{03B9}', HGK_BREVE | HGK_GRAVE ),
    /* EB45 */ ( '\u{03B9}', HGK_BREVE | HGK_SMOOTH | HGK_GRAVE ),
    /* EB46 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB47 */ ( '\u{03B9}', HGK_BREVE | HGK_ROUGH | HGK_ACUTE ),
    /* EB48 */ ( '\u{03B9}', HGK_BREVE | HGK_ROUGH | HGK_GRAVE ),
    /* EB49 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB4A */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB4B */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB4C */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB4D */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB4E */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB4F */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB50 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB51 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB52 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB53 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB54 */ ( '\u{03B9}', HGK_MACRON | HGK_SMOOTH | HGK_GRAVE ),
    /* EB55 */ ( '\u{03B9}', HGK_MACRON | HGK_ROUGH | HGK_GRAVE ),
    /* EB56 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB57 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB58 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB59 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB5A */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB5B */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB5C */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB5D */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB5E */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB5F */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB60 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB61 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB62 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB63 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB64 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB65 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB66 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB67 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB68 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB69 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB6A */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB6B */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB6C */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB6D */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB6E */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB6F */ ( '\u{03C5}', HGK_MACRON | HGK_GRAVE ),
    /* EB70 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB71 */ ( '\u{03C5}', HGK_MACRON | HGK_SMOOTH | HGK_GRAVE ),
    /* EB72 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB73 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB74 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB75 */ ( '\u{03C5}', HGK_MACRON | HGK_ROUGH | HGK_GRAVE ),
    /* EB76 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB77 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB78 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB79 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB7A */ ( '\u{03C5}', HGK_MACRON | HGK_ACUTE ),
    /* EB7B */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB7C */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB7D */ ( '\u{03C5}', HGK_MACRON | HGK_SMOOTH ),
    /* EB7E */ ( '\u{03C5}', HGK_MACRON | HGK_ROUGH ),
    /* EB7F */ ( '\u{03C5}', HGK_MACRON | HGK_SMOOTH | HGK_ACUTE ),
    /* EB80 */ ( '\u{03C5}', HGK_MACRON | HGK_ROUGH | HGK_ACUTE ),

    /* EB81 */ ( '\u{03C5}', HGK_BREVE | HGK_ACUTE ),
    /* EB82 */ ( '\u{03C5}', HGK_BREVE | HGK_ROUGH ),
    /* EB83 */ ( '\u{03C5}', HGK_BREVE | HGK_GRAVE ),
    /* EB84 */ ( '\u{03C5}', HGK_BREVE | HGK_SMOOTH ),
    /* EB85 */ ( '\u{03C5}', HGK_BREVE | HGK_SMOOTH | HGK_ACUTE ),
    /* EB86 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB87 */ ( '\u{0000}', HGK_NO_DIACRITICS),
    /* EB88 */ ( '\u{03C5}', HGK_BREVE | HGK_SMOOTH | HGK_GRAVE ),
    /* EB89 */ ( '\u{03C5}', HGK_BREVE | HGK_ROUGH | HGK_ACUTE ),
    /* EB8A */ ( '\u{03C5}', HGK_BREVE | HGK_ROUGH | HGK_GRAVE )
];

static GREEK_UPPER: &'static [char] = &[
'\u{0391}',
'\u{0392}',
'\u{03A8}',
'\u{0394}',
'\u{0395}',
'\u{03A6}',
'\u{0393}',
'\u{0397}',
'\u{0399}',
'\u{039E}',
'\u{039A}',
'\u{039B}',
'\u{039C}',
'\u{039D}',
'\u{039F}',
'\u{03A0}',
'\u{03DC}',
'\u{03A1}',
'\u{03A3}',
'\u{03A4}',
'\u{0398}',
'\u{03A9}',
'\u{00B7}',
'\u{03A7}',
'\u{03A5}',
'\u{0396}'
];

static GREEK_LOWER: &'static [char] = &[
'\u{03B1}',
'\u{03B2}',
'\u{03C8}',
'\u{03B4}',
'\u{03B5}',
'\u{03C6}',
'\u{03B3}',
'\u{03B7}',
'\u{03B9}',
'\u{03BE}',
'\u{03BA}',
'\u{03BB}',
'\u{03BC}',
'\u{03BD}',
'\u{03BF}',
'\u{03C0}',
'\u{03DD}',
'\u{03C1}',
'\u{03C3}',
'\u{03C4}',
'\u{03B8}',
'\u{03C9}',
'\u{03C2}',
'\u{03C7}',
'\u{03C5}',
'\u{03B6}'
];

pub fn transliterate(input:usize) -> char {
    if input >= 0x0061 && input <= 0x007A {
        return GREEK_LOWER[input - 0x0061];
    }
    else if input >= 0x0041 && input <= 0x005A {
        return GREEK_UPPER[input - 0x0041];
    }
    else {
        return '\u{0000}';
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn mytest() {
        /*
        let s = "ἄβί".to_string();
        let a = s.nfd();
        assert_eq!(a.count(), 6);
        */
        let z4 = "\u{EAF0}".nfd();
        //println!("test pua: {}", z4);
        assert_eq!("\u{EAF0}".nfd().next(), Some('\u{EAF0}'));

        assert_eq!(transliterate(0x0000), '\u{0000}');
        assert_eq!(transliterate(0x0040), '\u{0000}');
        assert_eq!(transliterate(0x0061), '\u{03B1}');
        assert_eq!(transliterate(0x007B), '\u{0000}');

        assert_eq!('α'.is_long_or_short(), true);
        assert_eq!('ι'.is_long_or_short(), true);
        assert_eq!('υ'.is_long_or_short(), true);
        assert_eq!('η'.is_long(), true);
        assert_eq!('ω'.is_long(), true);
        assert_eq!('ε'.is_short(), true);
        assert_eq!('ο'.is_short(), true);

        let _aa = HGKLetter::from_str("\u{EAF0}");

        let a2 = HGKLetter::from_str("\u{03B1}\u{0301}");
        assert_eq!(a2.diacritics & HGK_ACUTE, HGK_ACUTE);
        assert_eq!(a2.letter, '\u{03B1}');
        let a3 = HGKLetter::from_str("\u{03AC}");
        assert_eq!(a3.diacritics & HGK_ACUTE, HGK_ACUTE);
        assert_eq!(a3.letter, '\u{03B1}');

        let mut s: HGKLetter = HGKLetter { letter: 'α', diacritics: HGK_ACUTE | HGK_GRAVE };
        assert_eq!(s.diacritics & HGK_ACUTE, HGK_ACUTE);
        assert_ne!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);

        s.toggle_diacritic(HGK_CIRCUMFLEX, true);
        assert_eq!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);
        //don't toggle off, if on_only is set
        s.toggle_diacritic(HGK_CIRCUMFLEX, true);
        assert_eq!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);
        //turn off
        s.toggle_diacritic(HGK_CIRCUMFLEX, false);
        assert_ne!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);

        assert_eq!(compose('A','\u{30a}'), Some('Å'));

        let s = "ÅΩ";
        let c = s.nfc().collect::<String>();
        assert_eq!(c, "ÅΩ");

    	assert_eq!(compose('\u{03B1}','\u{0301}'), Some('\u{03AC}'));
    	assert_eq!(compose('\u{03B1}','\u{0301}'), Some('\u{03AC}'));
    	assert_eq!('a','a');

        let a = "\u{03B1}\u{0301}";
        let b = "\u{03AC}";
        assert_ne!(a, b);

        let s = String::from("ἄ");
        let _v: Vec<char> = s.chars().collect();

        let a4 = toggle_diacritic_str("α", HGK_ACUTE, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a4, "\u{03AC}");//ά");
        let a6 = toggle_diacritic_str("ὰ", HGK_ACUTE, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a6, "\u{03AC}");//ά");
        let a5 = toggle_diacritic_str("α", HGK_ACUTE, false, HGKUnicode_Mode::CombiningOnly);
        assert_eq!(a5, "\u{03B1}\u{0301}");
        let a7 = toggle_diacritic_str("α", HGK_CIRCUMFLEX, false, HGKUnicode_Mode::CombiningOnly);
        assert_eq!(a7, "\u{03B1}\u{0342}");
        let a8 = toggle_diacritic_str("α", HGK_CIRCUMFLEX, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a8, "\u{1FB6}");

        let a9 = toggle_diacritic_str("ε", HGK_CIRCUMFLEX, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a9, "ε");
    }
}
