//#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate alloc;
use alloc::string::String;
//use alloc::string::ToString;

use core::cmp;

//extern crate tinyvec;
//use tinyvec::TinyVec;

//use core::fmt::Display;
extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub use crate::tables::*;
mod tables;

const MACRON_AND_SMOOTH:u32 = HGK_MACRON | HGK_SMOOTH;
const MACRON_AND_SMOOTH_AND_ACUTE:u32 = HGK_MACRON | HGK_SMOOTH | HGK_ACUTE;
const MACRON_AND_SMOOTH_AND_GRAVE:u32 = HGK_MACRON | HGK_SMOOTH | HGK_GRAVE;
const MACRON_AND_ROUGH:u32 = HGK_MACRON | HGK_ROUGH;
const MACRON_AND_ROUGH_AND_ACUTE:u32 = HGK_MACRON | HGK_ROUGH | HGK_ACUTE;
const MACRON_AND_ROUGH_AND_GRAVE:u32 = HGK_MACRON | HGK_ROUGH | HGK_GRAVE;
const MACRON_AND_ACUTE:u32 = HGK_MACRON | HGK_ACUTE;
const MACRON_AND_GRAVE:u32 = HGK_MACRON | HGK_GRAVE;

const BREVE_AND_SMOOTH:u32 = HGK_BREVE | HGK_SMOOTH;
const BREVE_AND_SMOOTH_AND_ACUTE:u32 = HGK_BREVE | HGK_SMOOTH | HGK_ACUTE;
const BREVE_AND_SMOOTH_AND_GRAVE:u32 = HGK_BREVE | HGK_SMOOTH | HGK_GRAVE;
const BREVE_AND_ROUGH:u32 = HGK_BREVE | HGK_ROUGH;
const BREVE_AND_ROUGH_AND_ACUTE:u32 = HGK_BREVE | HGK_ROUGH | HGK_ACUTE;
const BREVE_AND_ROUGH_AND_GRAVE:u32 = HGK_BREVE | HGK_ROUGH | HGK_GRAVE;
const BREVE_AND_ACUTE:u32 = HGK_BREVE | HGK_ACUTE;
const BREVE_AND_GRAVE:u32 = HGK_BREVE | HGK_GRAVE;

fn get_pua_index(letter:char, diacritics:u32) -> i32 {
    //turn off iota subscript and underdot temporarily 
    //since these are added as combining diacritics later
    let i = match (diacritics & !HGK_IOTA_SUBSCRIPT) & !HGK_UNDERDOT {
        MACRON_AND_SMOOTH           => 0,
        MACRON_AND_SMOOTH_AND_ACUTE => 1,
        MACRON_AND_SMOOTH_AND_GRAVE => 2,
        MACRON_AND_ROUGH            => 3,
        MACRON_AND_ROUGH_AND_ACUTE  => 4,
        MACRON_AND_ROUGH_AND_GRAVE  => 5,
        MACRON_AND_ACUTE            => 6,
        MACRON_AND_GRAVE            => 7,
        BREVE_AND_SMOOTH            => 8,
        BREVE_AND_SMOOTH_AND_ACUTE  => 9,
        BREVE_AND_SMOOTH_AND_GRAVE  => 10,
        BREVE_AND_ROUGH             => 11,
        BREVE_AND_ROUGH_AND_ACUTE   => 12,
        BREVE_AND_ROUGH_AND_GRAVE   => 13,
        BREVE_AND_ACUTE             => 14,
        BREVE_AND_GRAVE             => 15,
        _                           => return -1,
    };
    
    match letter {
        'α' => i,
        'ι' => i + 16,
        'υ' => i + 32,
        _ => -1,
    }
}

#[derive(Copy, Clone)]
pub enum HgkUnicodeMode {
    Precomposed,
    CombiningOnly,
    PrecomposedPUA
}

#[derive(PartialEq, Debug)]
pub struct HGKLetter {
    letter: char,
    diacritics: u32
}

pub trait GreekLetters {
    fn gkletters<'a>(&'a self) -> GreekLetterHolder;
}

impl GreekLetters for str {
    #[inline]
    fn gkletters<'a>(&'a self) -> GreekLetterHolder<'a> {
        new_gkletters(self)
    }
}

#[derive(Clone, Debug)]
pub struct GreekLetterHolder<'a> {
    string: &'a str,
    cursor: GreekLetterCursor,
    cursor_back: GreekLetterCursor,
}

impl<'a> GreekLetterHolder<'a> {
    /*
    #[inline]

    /// View the underlying data (the part yet to be iterated) as a slice of the original string.
    ///
    /// ```rust
    /// # use unicode_segmentation::UnicodeSegmentation;
    /// let mut iter = "abc".graphemes(true);
    /// assert_eq!(iter.as_str(), "abc");
    /// iter.next();
    /// assert_eq!(iter.as_str(), "bc");
    /// iter.next();
    /// iter.next();
    /// assert_eq!(iter.as_str(), "");
    /// ```
    ///pub fn as_str(&self) -> &'a str {
    ///    &self.string[self.cursor.cur_cursor()..self.cursor_back.cur_cursor()]
    ///}
    */
}

impl<'a> Iterator for GreekLetterHolder<'a> {
    type Item = HGKLetter;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let slen = self.cursor_back.cur_cursor() - self.cursor.cur_cursor();
        (cmp::min(slen, 1), Some(slen))
    }

    #[inline]
    fn next(&mut self) -> Option<HGKLetter> {
        let start = self.cursor.cur_cursor();
        if start == self.cursor_back.cur_cursor() {
            return None;
        }

        let r = self.cursor.next_boundary(&self.string, 0);
        //println!("next: {} {} {}", start, self.cursor_back.cur_cursor(), r.as_ref().unwrap().as_ref().unwrap().letter);

        Some(r.unwrap().unwrap())
    }
}
/*
impl<'a> DoubleEndedIterator for GreekLetterHolder<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        let end = self.cursor_back.cur_cursor();
        if end == self.cursor.cur_cursor() {
            return None;
        }
        let prev = self.cursor_back.prev_boundary(self.string, 0).unwrap().unwrap();
        Some(&self.string[prev..end])
    }
}
*/

#[inline]
pub fn new_gkletters<'b>(s: &'b str) -> GreekLetterHolder<'b> {
    let len = s.len();
    GreekLetterHolder {
        string: s,
        cursor: GreekLetterCursor::new(0, len),
        cursor_back: GreekLetterCursor::new(len, len),
    }
}

#[derive(Clone, Debug)]
pub struct GreekLetterCursor {
    // Current cursor position.
    offset: usize,
    // Total length of the string.
    len: usize
}

/// An error return indicating that not enough content was available in the
/// provided chunk to satisfy the query, and that more content must be provided.
#[derive(PartialEq, Eq, Debug)]
pub enum GreekLetterError {
    /// More pre-context is needed. The caller should call `provide_context`
    /// with a chunk ending at the offset given, then retry the query. This
    /// will only be returned if the `chunk_start` parameter is nonzero.
    PreContext(usize),

    /// When requesting `prev_boundary`, the cursor is moving past the beginning
    /// of the current chunk, so the chunk before that is requested. This will
    /// only be returned if the `chunk_start` parameter is nonzero.
    PrevChunk,

    /// When requesting `next_boundary`, the cursor is moving past the end of the
    /// current chunk, so the chunk after that is requested. This will only be
    /// returned if the chunk ends before the `len` parameter provided on
    /// creation of the cursor.
    NextChunk,  // requesting chunk following the one given

    /// An error returned when the chunk given does not contain the cursor position.
    InvalidOffset,
}


impl GreekLetterCursor {
    pub fn new(offset: usize, len: usize) -> GreekLetterCursor {
        GreekLetterCursor {
            offset: offset,
            len: len
        }
    }

    // Not sure I'm gonna keep this, the advantage over new() seems thin.
    /// Set the cursor to a new location in the same string.
    pub fn set_cursor(&mut self, offset: usize) {
        if offset != self.offset {
            self.offset = offset;
        }
    }

    #[inline]
    /// The current offset of the cursor. Equal to the last value provided to
    /// `new()` or `set_cursor()`, or returned from `next_boundary()` or
    /// `prev_boundary()`.
    pub fn cur_cursor(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn next_boundary(&mut self, chunk: &str, chunk_start: usize) -> Result<Option<HGKLetter>, GreekLetterError> {

        if self.offset >= self.len {
            //println!("herehere: {}", self.offset);
            return Ok(None);
        }
        let mut the_letter = '\u{0000}';
        let mut diacritics:u32 = 0;

        let mut iter = chunk[self.offset - chunk_start..].chars(); //nfd();//
        let mut ch = iter.next().unwrap();
        //println!("next boundary: offset: {} {}", self.offset, ch);
        
        loop {
                if the_letter == '\u{0000}' && !hgk_is_combining(ch) {
                    if ch as u32 >= 0x1F00 && ch as u32 <= 0x1FFF {
                        //extended greek conversion
                        the_letter = GREEK_EXTENDED[ch as usize - 0x1F00].0;
                        diacritics = GREEK_EXTENDED[ch as usize - 0x1F00].1;
                    }
                    else if ch as u32 >= 0xEAF0 && ch as u32 <= 0xEB8A {
                        //PUA conversion
                        the_letter = GREEK_PUA[ch as usize - 0xEAF0].0;
                        diacritics = GREEK_PUA[ch as usize - 0xEAF0].1;
                    }
                    else {
                       the_letter = ch;
                    }
                }
                else if hgk_is_combining(ch) {
                    match ch {
                        '\u{0300}' => diacritics |= HGK_GRAVE,
                        '\u{0301}' => diacritics |= HGK_ACUTE,
                        '\u{0304}' => diacritics |= HGK_MACRON,
                        '\u{0306}' => diacritics |= HGK_BREVE,
                        '\u{0308}' => diacritics |= HGK_DIAERESIS,
                        '\u{0313}' => diacritics |= HGK_SMOOTH,
                        '\u{0314}' => diacritics |= HGK_ROUGH,
                        '\u{0323}' => diacritics |= HGK_UNDERDOT,
                        '\u{0342}' => diacritics |= HGK_CIRCUMFLEX,
                        '\u{0345}' => diacritics |= HGK_IOTA_SUBSCRIPT,
                        _ => {}
                    }
                }
                else {
                    //self.offset += ch.len_utf8();
                    //else boundary character, return
                    return Ok(Some(HGKLetter{letter:the_letter, diacritics:diacritics}));
                }

                self.offset += ch.len_utf8();
                if let Some(next_ch) = iter.next() {        
                    ch = next_ch;

                } else if self.offset == self.len {
                    //at the end
                    //println!("herehere2: {}", self.offset);
                    //return Ok(None);
                    return Ok(Some(HGKLetter{letter:the_letter, diacritics:diacritics}));
                }
                else {
                    return Ok(None);
                }
            }    
        }
    
/*
    /// Find the previous boundary after the current cursor position. Only a part
    /// of the string need be supplied. If the chunk is incomplete, then this
    /// method might return `GreekLetterError::PreContext` or
    /// `GreekLetterError::PrevChunk`. In the former case, the caller should
    /// call `provide_context` with the requested chunk, then retry. In the
    /// latter case, the caller should provide the chunk preceding the one
    /// given, then retry.
    ///
    /// See `is_boundary` for expectations on the provided chunk.
    ///
    /// ```rust
    /// # use unicode_segmentation::GreekLetterCursor;
    /// let flags = "\u{1F1F7}\u{1F1F8}\u{1F1EE}\u{1F1F4}";
    /// let mut cursor = GreekLetterCursor::new(12, flags.len(), false);
    /// assert_eq!(cursor.prev_boundary(flags, 0), Ok(Some(8)));
    /// assert_eq!(cursor.prev_boundary(flags, 0), Ok(Some(0)));
    /// assert_eq!(cursor.prev_boundary(flags, 0), Ok(None));
    /// ```
    ///
    /// And an example that uses partial strings (note the exact return is not
    /// guaranteed, and may be `PrevChunk` or `PreContext` arbitrarily):
    ///
    /// ```rust
    /// # use unicode_segmentation::{GreekLetterCursor, GreekLetterError};
    /// let s = "abcd";
    /// let mut cursor = GreekLetterCursor::new(4, s.len(), false);
    /// assert_eq!(cursor.prev_boundary(&s[2..4], 2), Ok(Some(3)));
    /// assert_eq!(cursor.prev_boundary(&s[2..4], 2), Err(GreekLetterError::PrevChunk));
    /// assert_eq!(cursor.prev_boundary(&s[0..2], 0), Ok(Some(2)));
    /// assert_eq!(cursor.prev_boundary(&s[0..2], 0), Ok(Some(1)));
    /// assert_eq!(cursor.prev_boundary(&s[0..2], 0), Ok(Some(0)));
    /// assert_eq!(cursor.prev_boundary(&s[0..2], 0), Ok(None));
    /// ```
    pub fn prev_boundary(&mut self, chunk: &str, chunk_start: usize) -> Result<Option<usize>, GreekLetterError> {
        if self.offset == 0 {
            return Ok(None);
        }
        if self.offset == chunk_start {
            return Err(GreekLetterError::PrevChunk);
        }
        let mut iter = chunk[..self.offset - chunk_start].chars().rev();
        let mut ch = iter.next().unwrap();
        loop {
            if self.offset == chunk_start {
                self.resuming = true;
                return Err(GreekLetterError::PrevChunk);
            }
            if self.resuming {
                self.cat_before = Some(self.grapheme_category(ch));
            } else {
                self.offset -= ch.len_utf8();
                self.cat_after = self.cat_before.take();
                self.state = GraphemeState::Unknown;
                if let Some(ris_count) = self.ris_count {
                    self.ris_count = if ris_count > 0 { Some(ris_count - 1) } else { None };
                }
                if let Some(prev_ch) = iter.next() {
                    ch = prev_ch;
                    self.cat_before = Some(self.grapheme_category(ch));
                } else if self.offset == 0 {
                    self.decide(true);
                } else {
                    self.resuming = true;
                    self.cat_after = Some(self.grapheme_category(ch));
                    return Err(GreekLetterError::PrevChunk);
                }
            }
            self.resuming = true;
            if self.is_boundary(chunk, chunk_start)? {
                self.resuming = false;
                return Ok(Some(self.offset));
            }
            self.resuming = false;
        }
    }
}
*/
}
/************************************************/


impl HGKLetter {
    fn from_str(l:&str) -> HGKLetter {
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
                match c {
                    '\u{0300}' => diacritics |= HGK_GRAVE,
                    '\u{0301}' => diacritics |= HGK_ACUTE,
                    '\u{0304}' => diacritics |= HGK_MACRON,
                    '\u{0306}' => diacritics |= HGK_BREVE,
                    '\u{0308}' => diacritics |= HGK_DIAERESIS,
                    '\u{0313}' => diacritics |= HGK_SMOOTH,
                    '\u{0314}' => diacritics |= HGK_ROUGH,
                    '\u{0323}' => diacritics |= HGK_UNDERDOT,
                    '\u{0342}' => diacritics |= HGK_CIRCUMFLEX,
                    '\u{0345}' => diacritics |= HGK_IOTA_SUBSCRIPT,
                    _ => break
                }
            }
        }
        
        HGKLetter { letter: bare_letter, diacritics: diacritics }
    }
/*
order:
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
    fn to_string(&self, unicode_mode:HgkUnicodeMode) -> String {
        let mut s = vec![self.letter];
        if (self.diacritics & HGK_MACRON) == HGK_MACRON {
            s.push('\u{0304}');
        }
        if (self.diacritics & HGK_BREVE) == HGK_BREVE {
            s.push('\u{0306}');
        }
        if (self.diacritics & HGK_DIAERESIS) == HGK_DIAERESIS {
            s.push('\u{0308}');
        }
        if (self.diacritics & HGK_ROUGH) == HGK_ROUGH {
            s.push('\u{0314}');
        }
        if (self.diacritics & HGK_SMOOTH) == HGK_SMOOTH {
            s.push('\u{0313}');
        }    
        if (self.diacritics & HGK_ACUTE) == HGK_ACUTE {
            s.push('\u{0301}');
        }
        if (self.diacritics & HGK_GRAVE) == HGK_GRAVE {
            s.push('\u{0300}');
        }
        if (self.diacritics & HGK_CIRCUMFLEX) == HGK_CIRCUMFLEX {
            s.push('\u{0342}');
        }
        if (self.diacritics & HGK_IOTA_SUBSCRIPT) == HGK_IOTA_SUBSCRIPT {
            s.push('\u{0345}');
        }
        if (self.diacritics & HGK_UNDERDOT) == HGK_UNDERDOT {
            s.push('\u{0323}');
        }
        match unicode_mode {
            HgkUnicodeMode::CombiningOnly => s.into_iter().collect::<String>(),
            HgkUnicodeMode::PrecomposedPUA => {
                let idx = get_pua_index(self.letter, self.diacritics);
                if (0..=GREEK_LOWER_PUA.len() as i32 - 1 ).contains(&idx) {
                    s.clear();
                    s.push( GREEK_LOWER_PUA[idx as usize] );

                    if (self.diacritics & HGK_IOTA_SUBSCRIPT) == HGK_IOTA_SUBSCRIPT {
                        s.push('\u{0345}');
                    }
                    if (self.diacritics & HGK_UNDERDOT) == HGK_UNDERDOT {
                        s.push('\u{0323}');
                    }
                    s.into_iter().collect::<String>() 
                }
                else {
                    s.into_iter().nfc().collect::<String>() 
                }
            },
            _ => s.into_iter().nfc().collect::<String>()
        }  
    }
/*
    fn to_string_pua(&self) -> String {

    }
*/
    fn toggle_diacritic(&mut self, d:u32, on_only:bool) {
        if !self.is_legal(d) {
            return;
        }

        if self.diacritics & d != d || on_only {
            self.diacritics |= d;
        }
        else {
            self.diacritics &= !d; //turn off: rust uses !, C uses ~
            //return;
        }

        //turn off clashing diacritics:
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
                self.diacritics &= !(HGK_ACUTE | HGK_GRAVE | HGK_MACRON | HGK_BREVE);
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
            HGK_UNDERDOT => { },
            _ => {
                debug_assert!(false, "Unknown Diacritic passed")
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
                self.letter.is_long_or_short() || self.letter.is_long()
            },
            HGK_MACRON => {
                self.letter.is_long_or_short()
            },
            HGK_BREVE => {
                self.letter.is_long_or_short()    
            },
            HGK_IOTA_SUBSCRIPT => {
                matches!(self.letter, 'α' | 'ω' | 'η') 
            },
            HGK_DIAERESIS => {
                matches!(self.letter, 'ι' | 'υ')                
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
        write!(f, "{}", self.to_string2(HgkUnicodeMode::Precomposed));
    }
}
*/
trait HGKIsLong {
    fn is_long(&self) -> bool;
}

impl HGKIsLong for char {
    fn is_long(&self) -> bool {
        matches!(self, 'η' | 'ω' | 'Η' | 'Ω')
    }
}

trait HGKIsShort {
    fn is_short(&self) -> bool;
}

impl HGKIsShort for char {
    fn is_short(&self) -> bool {
        matches!(self, 'ε' | 'ο' | 'Ε' | 'Ο')
    }
}

trait HGKIsLongOrShort {
    fn is_long_or_short(&self) -> bool;
}

impl HGKIsLongOrShort for char {
    fn is_long_or_short(&self) -> bool {
        matches!(self, 'α' | 'ι' | 'υ' | 'Α' | 'Ι' | 'Υ')
    }
}

trait HGKIsGreekVowel {
    fn is_greek_vowel(&self) -> bool;
}

impl HGKIsGreekVowel for char {
    fn is_greek_vowel(&self) -> bool {
        //let letter2 = self.to_lowercase();
        matches!(self, 'α' | 'ε' | 'η' | 'ι' | 'ο' | 'υ' | 'ω' | 'Α' | 'Ε' | 'Η' | 'Ι' | 'Ο' | 'Υ' | 'Ω')
    }
}

pub fn hgk_strip_diacritics(l:&str) -> String {
    //let b = l.gkletters();
    //println!("num: {}", b.collect::<Vec<HGKLetter>>().len() );
    l.gkletters().map(|a| HGKLetter{letter:a.letter, diacritics:0}.to_string(HgkUnicodeMode::PrecomposedPUA)).collect::<String>()
}

pub fn hgk_convert(l:&str, mode:HgkUnicodeMode) -> String {
    //let b = l.gkletters();
    //println!("num: {}", b.collect::<Vec<HGKLetter>>().len() );
    l.gkletters().map(|a| a.to_string(mode)).collect::<String>()
}

pub fn hgk_toggle_diacritic_str(l:&str, d:u32, on_only:bool, mode:HgkUnicodeMode) -> String {
    let mut letter = HGKLetter::from_str(l);
    letter.toggle_diacritic(d, on_only);
    letter.to_string(mode)
}

pub fn hgk_compare(a:&str, b:&str, compare_type:u32) -> i32 {
    let mut a1 = a.gkletters();
    let mut b1 = b.gkletters();

    let mut a_letter:Option<HGKLetter>;
    let mut b_letter:Option<HGKLetter>;

    loop  {
        a_letter = a1.next();
        b_letter = b1.next();
        if a_letter.is_none() || b_letter.is_none() {
            break;
        }

        /*
        //skip non-greek chars if option is set
        if ((compareType & _HK_IGNORE_UNKNOWN_CHARS) == _HK_IGNORE_UNKNOWN_CHARS && type1 == NOCHAR) {
            continue;
        }
        else if ((compareType & _HK_IGNORE_UNKNOWN_CHARS) == _HK_IGNORE_UNKNOWN_CHARS && type2 == NOCHAR) {
            continue;
        }
        */

        let lettera = a_letter.as_ref().unwrap().letter as usize;
        let letterb = b_letter.as_ref().unwrap().letter as usize;

        //if one or both characters are out of the greek range
        if (lettera < 0x0370 || lettera > 0x03FF) && (letterb < 0x0370 || letterb > 0x03FF) {
            if lettera < letterb {
                return -1;
            }
            else if lettera > letterb {
                return 1;
            }
            else {
                return 0;
            }
        }
        else if lettera < 0x0370 || lettera > 0x03FF { //non-greek sorts before greek 
            return -1;
        }
        else if letterb < 0x0370 || letterb > 0x03FF { //non-greek sorts before greek 
            return 1;
        }

        let a_sort:u32 = GREEK_BASIC[lettera - 0x0370].2;
        let b_sort:u32 = GREEK_BASIC[letterb - 0x0370].2;

        //if one letter sorts less than the other
        if a_sort < b_sort {
            return -1;
        }
        else if a_sort > b_sort {
            return 1;
        }

        if (a_letter.as_ref().unwrap().diacritics & !compare_type) != (b_letter.as_ref().unwrap().diacritics & !compare_type) {
            if (a_letter.unwrap().diacritics & !compare_type) < (b_letter.unwrap().diacritics & !compare_type) {
                return -1;
            }
            else {
                return 1;
            }
        }
    }
    //here we have reached the end of one or both strings and they are still completely equal

    if a_letter.is_none() && b_letter.is_none() //both at end
    {
        return 0;
    }
    else if a_letter.is_none() //1 at end
    {
        return -1;
    }
    else //2 at end
    {
        return 1;
    }
}

#[inline]
pub fn hgk_is_combining(c:char) -> bool {
    match c {
        '\u{0300}' => true,
        '\u{0301}' => true,
        '\u{0304}' => true,
        '\u{0306}' => true,
        '\u{0308}' => true,
        '\u{0313}' => true,
        '\u{0314}' => true,
        '\u{0323}' => true,
        '\u{0342}' => true,
        '\u{0345}' => true,
        _ => { false }
    }
}

/*
fn accent_recessive(s:&str) -> String {

}
*/
pub fn hgk_transliterate(input:usize) -> char {
    if (0x0061..=0x007A).contains(&input) {
        GREEK_LOWER[input - 0x0061]
    }
    else if (0x0041..=0x005A).contains(&input) {
        GREEK_UPPER[input - 0x0041]
    }
    else {
        '\u{0000}'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_normalization::char::compose;
    use alloc::vec::Vec;
    use csv;
    use std::error::Error;
    use std::path::Path;

    fn docsvtest() -> Result<(), Box<dyn Error>> {
        //println!("{:?}", env::current_dir().unwrap());
        let csvfile = "gktest.csv";
        if !Path::new(csvfile).is_file() {
            Err("CSV tests file does not exist")? //or: return Err("Bad request".into());
        }

        let mut rdr = csv::Reader::from_path(csvfile)?; //Reader::from_reader(io::stdin());
        let mut line_number = 2; //start on line 2 because header row
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the error here.
            let record = result?;

            let diacritic = match record[1].trim() {
                //"none" => HGK_NO_DIACRITICS,
                "rough" => HGK_ROUGH,
                "smooth" => HGK_SMOOTH,
                "acute" => HGK_ACUTE,
                "grave" => HGK_GRAVE,
                "circumflex" => HGK_CIRCUMFLEX,
                "macron" => HGK_MACRON,
                "breve" => HGK_BREVE,
                "iotasub" => HGK_IOTA_SUBSCRIPT,
                "diaeresis" => HGK_DIAERESIS,
                "underdot" => HGK_UNDERDOT,
                _ => panic!("Invalid diacritic on line: {}.", line_number)
            };

            let only_on = match record[2].trim() {
                "onlyon" => true,
                "toggleoff" => false,
                _ => panic!("Invalid toggle off on line: {}.", line_number)
            };

            let mode = match record[3].trim() {
                "CombiningOnly" => HgkUnicodeMode::CombiningOnly,
                "PrecomposedPUA" => HgkUnicodeMode::PrecomposedPUA,
                "Precomposed" => HgkUnicodeMode::Precomposed,
                _ => panic!("Invalid unicode mode on line: {}.", line_number)
            };

            let is_equal = match record[5].trim() {
                "equal" => true,
                "notequal" => false,
                _ => panic!("Invalid equal on line: {}.", line_number)
            };

            if is_equal {
                assert_eq!(hgk_toggle_diacritic_str(&hex_to_string(&record[0]), diacritic, only_on, mode), hex_to_string(&record[4]), "Error on line: {}.", line_number);
            }
            else {
                assert_ne!(hgk_toggle_diacritic_str(&hex_to_string(&record[0]), diacritic, only_on, mode), hex_to_string(&record[4]), "Error on line: {}.", line_number);
            }
            line_number += 1;
        }

        Ok(())
    }

    //make string from utf16 hex codepoints
    fn hex_to_string(s:&str) -> String {
        //https://stackoverflow.com/questions/3408706/hexadecimal-string-to-byte-array-in-c
        let b = hex::decode(s.replace(" ", "")).unwrap();

        let res: Vec<u16> = b
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_be_bytes([a[0], a[1]]))
        .collect();

        String::from_utf16( res.as_slice() ).unwrap()
    }

    #[test]
    fn test_compare() {
        assert_eq!( hgk_compare("α", "α", 0), 0);
        assert_eq!( hgk_compare("α", "Α", 0), 0);
        assert_eq!( hgk_compare("Α", "Α", 0), 0);
        assert_eq!( hgk_compare("α", "β", 0), -1);
        assert_eq!( hgk_compare("β", "α", 0), 1);
        assert_eq!( hgk_compare("β", "ἄ", 0), 1);

        assert_eq!( hgk_compare("ω", "ω", 0), 0);
        assert_eq!( hgk_compare("α", "ω", 0), -1);
        assert_eq!( hgk_compare("ω", "α", 0), 1);


        assert_eq!( hgk_compare("αβ", "α", 0), 1);
        assert_eq!( hgk_compare("α", "αβ", 0), -1);

        assert_eq!( hgk_compare("αβ", "β", 0), -1);
        assert_eq!( hgk_compare("β", "αβ", 0), 1);
    }

    #[test]
    fn csv_tests() {
        match docsvtest() {
            Ok(()) => (),
            Err(error) => panic!("Error: {:?}", error)
        };
    }

    #[test]
    fn mytest() {
        //println!("{:?}", env::current_dir().unwrap());

        assert_eq!(hex_to_string("03B1 0304 03B2"), "α\u{0304}β");


        let s = "α\u{0304}\u{0313}\u{0301}βα\u{0313}\u{0301}";//"\u{EB07}βἄ";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'β', diacritics:0},HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "α\u{0304}\u{0313}\u{0301}βἄ";//"\u{EB07}βἄ";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'β', diacritics:0},HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}βἄ";//"ᾱ̓́βἄ";//
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'β', diacritics:0},HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{1F04}βἄ";//"ᾱ̓́βἄ";//
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_SMOOTH},HGKLetter{letter:'β', diacritics:0},HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        
        let mut aaa = "άβγ".gkletters();
        assert_eq!(aaa.next().unwrap().letter, 'α');
        assert_eq!(aaa.next().unwrap().letter, 'β');
        assert_eq!(aaa.next().unwrap().letter, 'γ');
        assert_eq!(aaa.next(), None);

        let s = "αβγ";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:0},HGKLetter{letter:'β', diacritics:0},HGKLetter{letter:'γ', diacritics:0} ];
        assert_eq!(g, b);

        let s = "ᾱ̓́";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}βἄ";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'β', diacritics:0},HGKLetter{letter:'α', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}βᾱ";
        let xxx = s.gkletters().map(|a| HGKLetter{letter:a.letter, diacritics:0} ).collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'α', diacritics:0},HGKLetter{letter:'β', diacritics:0},HGKLetter{letter:'α', diacritics:0} ];
        assert_eq!(xxx, b);

        let s = "\u{EB07}βἄ";
        let xxx = s.gkletters().map(|a| HGKLetter{letter:a.letter, diacritics:0}.to_string(HgkUnicodeMode::PrecomposedPUA)).collect::<String>();
        assert_eq!(xxx, "αβα");

        assert_eq!( hgk_strip_diacritics("ἄβ"), "αβ" );
        assert_eq!( hgk_strip_diacritics("\u{EB07}"), "α" );
        assert_eq!( hgk_strip_diacritics("α\u{0304}\u{0313}\u{0301}"), "α" );
        
        assert_eq!( hgk_convert("\u{EB07}", HgkUnicodeMode::CombiningOnly), "α\u{0304}\u{0313}\u{0301}");
        assert_eq!( hgk_convert("α\u{0304}\u{0313}\u{0301}", HgkUnicodeMode::PrecomposedPUA), "\u{EB07}");

        /*


        let s = "ἄβί".to_string();
        let a = s.nfd();
        assert_eq!(a.count(), 6);
        */
        //let z4 = "\u{EAF0}".nfd();
        //println!("test pua: {}", z4);

        //let str = "ἄλφά";
        //let str2 = str.nfd().chars().iter().filter(|x| !unicode_normalization::char::is_combining_mark(x))

        assert_eq!(GREEK_LOWER_PUA.len() as i32 - 1, 47);

        assert_eq!(MACRON_AND_SMOOTH, HGK_MACRON | HGK_SMOOTH);

        assert_eq!("\u{EAF0}".nfd().next(), Some('\u{EAF0}'));
        assert_eq!("\u{EAF0}".nfd().count(), 1);

        assert_eq!(hgk_transliterate(0x0000), '\u{0000}');
        assert_eq!(hgk_transliterate(0x0040), '\u{0000}');
        assert_eq!(hgk_transliterate(0x0061), '\u{03B1}');
        assert_eq!(hgk_transliterate(0x007B), '\u{0000}');

        assert_eq!('α'.is_long_or_short(), true);
        assert_eq!('α'.is_long(), false);
        assert_eq!('α'.is_short(), false);
        assert_eq!('ε'.is_long_or_short(), false);
        assert_eq!('ε'.is_long(), false);
        assert_eq!('ε'.is_short(), true);
        assert_eq!('η'.is_long_or_short(), false);
        assert_eq!('η'.is_long(), true);
        assert_eq!('η'.is_short(), false);
        assert_eq!('ι'.is_long_or_short(), true);
        assert_eq!('ι'.is_long(), false);
        assert_eq!('ι'.is_short(), false);
        assert_eq!('ο'.is_long_or_short(), false);
        assert_eq!('ο'.is_long(), false);
        assert_eq!('ο'.is_short(), true);
        assert_eq!('υ'.is_long_or_short(), true);
        assert_eq!('υ'.is_long(), false);
        assert_eq!('υ'.is_short(), false);
        assert_eq!('ω'.is_long_or_short(), false);
        assert_eq!('ω'.is_long(), true);
        assert_eq!('ω'.is_short(), false);

        assert_eq!('Α'.is_long_or_short(), true);
        assert_eq!('Α'.is_long(), false);
        assert_eq!('Α'.is_short(), false);
        assert_eq!('Ε'.is_long_or_short(), false);
        assert_eq!('Ε'.is_long(), false);
        assert_eq!('Ε'.is_short(), true);
        assert_eq!('Η'.is_long_or_short(), false);
        assert_eq!('Η'.is_long(), true);
        assert_eq!('Η'.is_short(), false);
        assert_eq!('Ι'.is_long_or_short(), true);
        assert_eq!('Ι'.is_long(), false);
        assert_eq!('Ι'.is_short(), false);
        assert_eq!('Ο'.is_long_or_short(), false);
        assert_eq!('Ο'.is_long(), false);
        assert_eq!('Ο'.is_short(), true);
        assert_eq!('Υ'.is_long_or_short(), true);
        assert_eq!('Υ'.is_long(), false);
        assert_eq!('Υ'.is_short(), false);
        assert_eq!('Ω'.is_long_or_short(), false);
        assert_eq!('Ω'.is_long(), true);
        assert_eq!('Ω'.is_short(), false);
        
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
 
        let mut a1 = HGKLetter::from_str("υ");
        assert_eq!(a1.letter, 'υ');
        assert_eq!(a1.diacritics, HGK_NO_DIACRITICS);
        a1.toggle_diacritic(HGK_MACRON, false);
        assert_eq!(a1.letter, 'υ');
        assert_eq!(a1.diacritics, HGK_MACRON);
        assert_eq!(get_pua_index(a1.letter, a1.diacritics), -1);
        assert_eq!(a1.to_string(HgkUnicodeMode::PrecomposedPUA), "\u{1FE1}");

        assert_eq!(hgk_toggle_diacritic_str("υ", HGK_MACRON, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{1FE1}");

        assert_eq!(hgk_toggle_diacritic_str("α", HGK_UNDERDOT, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{03B1}\u{0323}");



        assert_eq!(hgk_toggle_diacritic_str("ἀ", HGK_MACRON, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{EB04}");
        assert_eq!(hgk_toggle_diacritic_str("ἄ", HGK_MACRON, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{EB07}");
        assert_eq!(hgk_toggle_diacritic_str("ὺ", HGK_BREVE, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{EB83}");
        assert_eq!(hgk_toggle_diacritic_str("α", HGK_ACUTE, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{03AC}");

        assert_eq!(hgk_toggle_diacritic_str("α", HGK_ACUTE, false, HgkUnicodeMode::Precomposed), 
            "\u{03AC}");//ά");
        assert_eq!(hgk_toggle_diacritic_str("ὰ", HGK_ACUTE, false, HgkUnicodeMode::Precomposed), 
            "\u{03AC}");//ά");
        assert_eq!(hgk_toggle_diacritic_str("α", HGK_ACUTE, false, HgkUnicodeMode::CombiningOnly), 
            "\u{03B1}\u{0301}");
        assert_eq!(hgk_toggle_diacritic_str("α", HGK_CIRCUMFLEX, false, HgkUnicodeMode::CombiningOnly), 
            "\u{03B1}\u{0342}");
        assert_eq!(hgk_toggle_diacritic_str("α", HGK_CIRCUMFLEX, false, HgkUnicodeMode::Precomposed), 
            "\u{1FB6}");
        assert_eq!(hgk_toggle_diacritic_str("ε", HGK_CIRCUMFLEX, false, HgkUnicodeMode::Precomposed), 
            "ε");
        assert_eq!(hgk_toggle_diacritic_str("ω", HGK_CIRCUMFLEX, false, HgkUnicodeMode::Precomposed), 
            "ῶ");
        assert_eq!(hgk_toggle_diacritic_str("ρ", HGK_ROUGH, false, HgkUnicodeMode::Precomposed), 
            "ῥ");
        assert_eq!(hgk_toggle_diacritic_str("Ρ", HGK_ROUGH, false, HgkUnicodeMode::Precomposed), 
            "Ῥ");
        assert_eq!(hgk_toggle_diacritic_str("ρ", HGK_SMOOTH, false, HgkUnicodeMode::Precomposed), 
            "ῤ");
        assert_eq!(hgk_toggle_diacritic_str("Ρ", HGK_SMOOTH, false, HgkUnicodeMode::Precomposed), 
            "Ρ\u{0313}"); //there is no precomposed capital rho with smooth breathing
        assert_eq!(hgk_toggle_diacritic_str("Ρ\u{0313}", HGK_SMOOTH, false, HgkUnicodeMode::Precomposed), 
            "Ρ");
        assert_eq!(hgk_toggle_diacritic_str("Ρ\u{0313}", HGK_ROUGH, false, HgkUnicodeMode::Precomposed), 
            "Ῥ");
        assert_eq!(hgk_toggle_diacritic_str("Ρ\u{0313}", HGK_ROUGH, false, HgkUnicodeMode::CombiningOnly), 
            "Ρ\u{0314}");
    }
}
