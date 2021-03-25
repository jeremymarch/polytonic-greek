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
    fn from_string() -> HGKLetter {
        return HGKLetter { letter: 'ἄ', diacritics: HGKDiacritics::ACUTE | HGKDiacritics::GRAVE };
    }

    fn to_string(&mut self, unicode_mode:HGKUnicode_Mode) -> String {
        return "αβγ".to_string();
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
                match self.letter {
                    'α' => true,
                    'ω' => true,
                    'ι' => true,
                    'υ' => true,
                    'η' => true,
                    _ => false
                }
            },
            HGKDiacritics::MACRON => {
                match self.letter {
                    'α' => true,
                    'ι' => true,
                    'υ' => true,
                    _ => false
                }                 
            },
            HGKDiacritics::BREVE => {
                match self.letter {
                    'α' => true,
                    'ι' => true,
                    'υ' => true,
                    _ => false
                }                  
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
            _ => return false
        }
    }
}

fn toggle_diacritic_str(l:&str, d:HGKDiacritics, on_only:bool) -> String {
    let mut diacritics: HGKDiacritics = HGKDiacritics::empty();

    for (i, c) in l.nfd().enumerate() {
        if i == 0 {
            if unicode_normalization::char::is_combining_mark(c) {
                assert!(false, "First char of letter is a combining mark??");
            }
        }
        else {
            if !unicode_normalization::char::is_combining_mark(c) {
                break;
            }
        }
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
    let mut l = HGKLetter { letter: l.chars().nth(0).unwrap(), diacritics: diacritics };
    l.toggle_diacritic(d, on_only);
    return l.to_string(HGKUnicode_Mode::Precomposed);
}

fn main() {

    let s = "ἄβί".to_string();
    let a = s.nfd();
    println!("blah {}", a.count() );

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

    if a == b
    {
    	println!("equal");
    }
    else
    {
    	println!("not equal");
    }

    let s = String::from("ἄ");
    let v: Vec<char> = s.chars().collect();



}
