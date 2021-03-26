extern crate unicode_normalization;
use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;

#[macro_use]
extern crate bitflags;

bitflags! {
    struct HGKDiacritics: u32 {
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

enum HGKUnicode_Mode {
    Precomposed,
    PrecomposedPUA,
    CombiningOnly
}

struct HGKLetter {
    letter: char,
    diacritics: HGKDiacritics
}

impl HGKLetter {
    fn from_str(l:&str) -> HGKLetter {
        let mut diacritics: HGKDiacritics = HGKDiacritics::empty();
        let mut bare_letter: char = '\u{0000}';
        for (i, c) in l.nfd().enumerate() {
            if i == 0 {
                if unicode_normalization::char::is_combining_mark(c) {
                    assert!(false, "First char of letter is a combining mark.");
                }
                bare_letter = c;
            }
            else {
                if !unicode_normalization::char::is_combining_mark(c) {
                    break;
                }
                else {
                    match c {
                        '\u{0300}' => diacritics |= HGKDiacritics::GRAVE,
                        '\u{0301}' => diacritics |= HGKDiacritics::ACUTE,
                        '\u{0304}' => diacritics |= HGKDiacritics::MACRON,
                        '\u{0306}' => diacritics |= HGKDiacritics::BREVE,
                        '\u{0308}' => diacritics |= HGKDiacritics::DIAERESIS,
                        '\u{0313}' => diacritics |= HGKDiacritics::SMOOTH,
                        '\u{0314}' => diacritics |= HGKDiacritics::ROUGH,
                        '\u{0323}' => diacritics |= HGKDiacritics::UNDERDOT,
                        '\u{0342}' => diacritics |= HGKDiacritics::CIRCUMFLEX,
                        '\u{0345}' => diacritics |= HGKDiacritics::IOTA_SUBSCRIPT,
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
        let mut s = self.letter.to_string();
        if (self.diacritics & HGKDiacritics::MACRON) == HGKDiacritics::MACRON {
            s = s + "\u{0304}";
        }
        if (self.diacritics & HGKDiacritics::BREVE) == HGKDiacritics::BREVE {
            s = s + "\u{0306}";
        }
        if (self.diacritics & HGKDiacritics::DIAERESIS) == HGKDiacritics::DIAERESIS {
            s = s + "\u{0308}";
        }
        if (self.diacritics & HGKDiacritics::ROUGH) == HGKDiacritics::ROUGH {
            s = s + "\u{0314}";
        }
        if (self.diacritics & HGKDiacritics::SMOOTH) == HGKDiacritics::SMOOTH {
            s = s + "\u{0313}";
        }    
        if (self.diacritics & HGKDiacritics::ACUTE) == HGKDiacritics::ACUTE {
            s = s + "\u{0301}";
        }
        if (self.diacritics & HGKDiacritics::GRAVE) == HGKDiacritics::GRAVE {
            s = s + "\u{0300}";
        }
        if (self.diacritics & HGKDiacritics::CIRCUMFLEX) == HGKDiacritics::CIRCUMFLEX {
            s = s + "\u{0342}";
        }
        if (self.diacritics & HGKDiacritics::IOTA_SUBSCRIPT) == HGKDiacritics::IOTA_SUBSCRIPT {
            s = s + "\u{0345}";
        }
        if (self.diacritics & HGKDiacritics::UNDERDOT) == HGKDiacritics::UNDERDOT {
            s = s + "\u{0323}";
        }
        match unicode_mode {
            HGKUnicode_Mode::CombiningOnly => return s,
            HGKUnicode_Mode::PrecomposedPUA => return s.nfc().collect::<String>(),
            _ => return s.nfc().collect::<String>()
        }
    }

    fn toggle_diacritic(&mut self, d:HGKDiacritics, on_only:bool) {
        if !self.isLegal(d) {
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
            HGKDiacritics::ROUGH => {
                self.diacritics &= !(HGKDiacritics::SMOOTH | HGKDiacritics::DIAERESIS);
            },
            HGKDiacritics::SMOOTH => {
                self.diacritics &= !(HGKDiacritics::ROUGH | HGKDiacritics::DIAERESIS);
            },
            HGKDiacritics::ACUTE => {
                self.diacritics &= !(HGKDiacritics::GRAVE | HGKDiacritics::CIRCUMFLEX);
            },
            HGKDiacritics::GRAVE => {
                self.diacritics &= !(HGKDiacritics::ACUTE | HGKDiacritics::CIRCUMFLEX);
            },
            HGKDiacritics::CIRCUMFLEX => {
                self.diacritics &= !(HGKDiacritics::ACUTE | HGKDiacritics::GRAVE);
            },
            HGKDiacritics::MACRON => {
                self.diacritics &= !(HGKDiacritics::BREVE | HGKDiacritics::CIRCUMFLEX);
            },
            HGKDiacritics::BREVE => {
                self.diacritics &= !(HGKDiacritics::MACRON | HGKDiacritics::CIRCUMFLEX | HGKDiacritics::IOTA_SUBSCRIPT);
            },
            HGKDiacritics::IOTA_SUBSCRIPT => {
                self.diacritics &= !(HGKDiacritics::BREVE);
            },
            HGKDiacritics::DIAERESIS => {
                self.diacritics &= !(HGKDiacritics::ROUGH | HGKDiacritics::SMOOTH);
            },
            //HGKDiacritics::UNDERDOT => { },
            _ => {
                assert!(false, "Unknown Diacritic passed")
            }
        }
    }

    fn isLegal(&mut self, d:HGKDiacritics) -> bool {
        match d {
            HGKDiacritics::ROUGH => {
                true
            },
            HGKDiacritics::SMOOTH => {
                true
            },
            HGKDiacritics::ACUTE => {
                true
            },
            HGKDiacritics::GRAVE => {
                true
            },
            HGKDiacritics::CIRCUMFLEX => {
                self.letter.is_long_or_short() | self.letter.is_long()
            },
            HGKDiacritics::MACRON => {
                self.letter.is_long_or_short()
            },
            HGKDiacritics::BREVE => {
                self.letter.is_long_or_short()    
            },
            HGKDiacritics::IOTA_SUBSCRIPT => {
                match self.letter {
                    'α' => true,
                    'ω' => true,
                    'η' => true,
                    _ => false
                } 
            },
            HGKDiacritics::DIAERESIS => {
                match self.letter {
                    'ι' => true,
                    'υ' => true,
                    _ => false
                }                
            },
            //HGKDiacritics::UNDERDOT => { },
            _ => false
        }
    }
}

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

fn toggle_diacritic_str(l:&str, d:HGKDiacritics, on_only:bool, mode:HGKUnicode_Mode) -> String {
    let mut letter = HGKLetter::from_str(l);
    letter.toggle_diacritic(d, on_only);
    return letter.to_string(mode);
}

fn main() {

    let s = "ἄβί".to_string();
    let a = s.nfd();
    assert_eq!(a.count(), 6);

    assert_eq!('α'.is_long_or_short(), true);
    assert_eq!('ι'.is_long_or_short(), true);
    assert_eq!('υ'.is_long_or_short(), true);
    assert_eq!('η'.is_long(), true);
    assert_eq!('ω'.is_long(), true);
    assert_eq!('ε'.is_short(), true);
    assert_eq!('ο'.is_short(), true);

    let a2 = HGKLetter::from_str("\u{03B1}\u{0301}");
    assert_eq!(a2.diacritics & HGKDiacritics::ACUTE, HGKDiacritics::ACUTE);
    assert_eq!(a2.letter, '\u{03B1}');
    let a3 = HGKLetter::from_str("\u{03AC}");
    assert_eq!(a3.diacritics & HGKDiacritics::ACUTE, HGKDiacritics::ACUTE);
    assert_eq!(a3.letter, '\u{03B1}');

    let mut s: HGKLetter = HGKLetter { letter: 'α', diacritics: HGKDiacritics::ACUTE | HGKDiacritics::GRAVE };
    assert_eq!(s.diacritics & HGKDiacritics::ACUTE, HGKDiacritics::ACUTE);
    assert_ne!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);

    s.toggle_diacritic(HGKDiacritics::CIRCUMFLEX, true);
    assert_eq!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);
    //don't toggle off, if on_only is set
    s.toggle_diacritic(HGKDiacritics::CIRCUMFLEX, true);
    assert_eq!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);
    //turn off
    s.toggle_diacritic(HGKDiacritics::CIRCUMFLEX, false);
    assert_ne!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);

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
    let v: Vec<char> = s.chars().collect();

    let a4 = toggle_diacritic_str("α", HGKDiacritics::ACUTE, false, HGKUnicode_Mode::Precomposed);
    assert_eq!(a4, "\u{03AC}");//ά");
    let a6 = toggle_diacritic_str("ὰ", HGKDiacritics::ACUTE, false, HGKUnicode_Mode::Precomposed);
    assert_eq!(a6, "\u{03AC}");//ά");
    let a5 = toggle_diacritic_str("α", HGKDiacritics::ACUTE, false, HGKUnicode_Mode::CombiningOnly);
    assert_eq!(a5, "\u{03B1}\u{0301}");
    let a7 = toggle_diacritic_str("α", HGKDiacritics::CIRCUMFLEX, false, HGKUnicode_Mode::CombiningOnly);
    assert_eq!(a7, "\u{03B1}\u{0342}");
    let a8 = toggle_diacritic_str("α", HGKDiacritics::CIRCUMFLEX, false, HGKUnicode_Mode::Precomposed);
    assert_eq!(a8, "\u{1FB6}");

    let a9 = toggle_diacritic_str("ε", HGKDiacritics::CIRCUMFLEX, false, HGKUnicode_Mode::Precomposed);
    assert_eq!(a9, "ε");
}
