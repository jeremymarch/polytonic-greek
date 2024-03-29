pub const HGK_NO_DIACRITICS: u32 = 0x000;
pub const HGK_ROUGH: u32 = 0x001;
pub const HGK_SMOOTH: u32 = 0x002;
pub const HGK_ACUTE: u32 = 0x004;
pub const HGK_GRAVE: u32 = 0x008;
pub const HGK_CIRCUMFLEX: u32 = 0x010;
pub const HGK_MACRON: u32 = 0x020;
pub const HGK_BREVE: u32 = 0x040;
pub const HGK_IOTA_SUBSCRIPT: u32 = 0x080;
pub const HGK_DIAERESIS: u32 = 0x100;
pub const HGK_UNDERDOT: u32 = 0x200;

pub(crate) const GREEK_PUA: &[(char, u32)] = &[
    /* EAF0 */ ('\u{03B1}', HGK_MACRON | HGK_GRAVE),
    /* EAF1 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EAF2 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EAF3 */ ('\u{03B1}', HGK_MACRON | HGK_SMOOTH | HGK_GRAVE),
    /* EAF4 */ ('\u{03B1}', HGK_MACRON | HGK_ROUGH | HGK_GRAVE),
    /* EAF5 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EAF6 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EAF7 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EAF8 */ ('\u{03B1}', HGK_BREVE | HGK_GRAVE),
    /* EAF9 */ ('\u{03B1}', HGK_BREVE | HGK_SMOOTH),
    /* EAFA */ ('\u{03B1}', HGK_BREVE | HGK_SMOOTH | HGK_GRAVE),
    /* EAFB */ ('\u{03B1}', HGK_BREVE | HGK_ROUGH | HGK_ACUTE),
    /* EAFC */ ('\u{03B1}', HGK_BREVE | HGK_ROUGH | HGK_GRAVE),
    /* EAFD */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EAFE */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EAFF */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB00 */ ('\u{03B1}', HGK_MACRON | HGK_ACUTE),
    /* EB01 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB02 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB03 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB04 */ ('\u{03B1}', HGK_MACRON | HGK_SMOOTH),
    /* EB05 */ ('\u{03B1}', HGK_MACRON | HGK_ROUGH),
    /* EB06 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB07 */ ('\u{03B1}', HGK_MACRON | HGK_SMOOTH | HGK_ACUTE),
    /* EB08 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB09 */ ('\u{03B1}', HGK_MACRON | HGK_ROUGH | HGK_ACUTE),
    /* EB0A */ ('\u{03B1}', HGK_BREVE | HGK_ACUTE),
    /* EB0B */ ('\u{03B1}', HGK_BREVE | HGK_ROUGH),
    /* EB0C */ ('\u{03B1}', HGK_BREVE | HGK_SMOOTH | HGK_ACUTE),
    /* EB0D */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB0E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB0F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB10 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB11 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB12 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB13 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB14 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB15 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB16 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB17 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB18 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB19 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB1A */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB1B */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB1C */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB1D */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB1E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB1F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB20 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB21 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB22 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB23 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB24 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB25 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB26 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB27 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB28 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB29 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB2A */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB2B */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB2C */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB2D */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB2E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB2F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB30 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB31 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB32 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB33 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB34 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB35 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB36 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB37 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB38 */ ('\u{03B9}', HGK_MACRON | HGK_GRAVE),
    /* EB39 */ ('\u{03B9}', HGK_MACRON | HGK_ACUTE),
    /* EB3A */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB3B */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB3C */ ('\u{03B9}', HGK_MACRON | HGK_SMOOTH),
    /* EB3D */ ('\u{03B9}', HGK_MACRON | HGK_SMOOTH | HGK_ACUTE),
    /* EB3E */ ('\u{03B9}', HGK_MACRON | HGK_ROUGH),
    /* EB3F */ ('\u{03B9}', HGK_MACRON | HGK_ROUGH | HGK_ACUTE),
    /* EB40 */ ('\u{03B9}', HGK_BREVE | HGK_ACUTE),
    /* EB41 */ ('\u{03B9}', HGK_BREVE | HGK_SMOOTH),
    /* EB42 */ ('\u{03B9}', HGK_BREVE | HGK_SMOOTH | HGK_ACUTE),
    /* EB43 */ ('\u{03B9}', HGK_BREVE | HGK_ROUGH),
    /* EB44 */ ('\u{03B9}', HGK_BREVE | HGK_GRAVE),
    /* EB45 */ ('\u{03B9}', HGK_BREVE | HGK_SMOOTH | HGK_GRAVE),
    /* EB46 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB47 */ ('\u{03B9}', HGK_BREVE | HGK_ROUGH | HGK_ACUTE),
    /* EB48 */ ('\u{03B9}', HGK_BREVE | HGK_ROUGH | HGK_GRAVE),
    /* EB49 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB4A */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB4B */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB4C */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB4D */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB4E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB4F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB50 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB51 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB52 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB53 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB54 */ ('\u{03B9}', HGK_MACRON | HGK_SMOOTH | HGK_GRAVE),
    /* EB55 */ ('\u{03B9}', HGK_MACRON | HGK_ROUGH | HGK_GRAVE),
    /* EB56 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB57 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB58 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB59 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB5A */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB5B */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB5C */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB5D */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB5E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB5F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB60 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB61 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB62 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB63 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB64 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB65 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB66 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB67 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB68 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB69 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB6A */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB6B */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB6C */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB6D */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB6E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB6F */ ('\u{03C5}', HGK_MACRON | HGK_GRAVE),
    /* EB70 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB71 */ ('\u{03C5}', HGK_MACRON | HGK_SMOOTH | HGK_GRAVE),
    /* EB72 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB73 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB74 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB75 */ ('\u{03C5}', HGK_MACRON | HGK_ROUGH | HGK_GRAVE),
    /* EB76 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB77 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB78 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB79 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB7A */ ('\u{03C5}', HGK_MACRON | HGK_ACUTE),
    /* EB7B */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB7C */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB7D */ ('\u{03C5}', HGK_MACRON | HGK_SMOOTH),
    /* EB7E */ ('\u{03C5}', HGK_MACRON | HGK_ROUGH),
    /* EB7F */ ('\u{03C5}', HGK_MACRON | HGK_SMOOTH | HGK_ACUTE),
    /* EB80 */ ('\u{03C5}', HGK_MACRON | HGK_ROUGH | HGK_ACUTE),
    /* EB81 */ ('\u{03C5}', HGK_BREVE | HGK_ACUTE),
    /* EB82 */ ('\u{03C5}', HGK_BREVE | HGK_ROUGH),
    /* EB83 */ ('\u{03C5}', HGK_BREVE | HGK_GRAVE),
    /* EB84 */ ('\u{03C5}', HGK_BREVE | HGK_SMOOTH),
    /* EB85 */ ('\u{03C5}', HGK_BREVE | HGK_SMOOTH | HGK_ACUTE),
    /* EB86 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB87 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* EB88 */ ('\u{03C5}', HGK_BREVE | HGK_SMOOTH | HGK_GRAVE),
    /* EB89 */ ('\u{03C5}', HGK_BREVE | HGK_ROUGH | HGK_ACUTE),
    /* EB8A */ ('\u{03C5}', HGK_BREVE | HGK_ROUGH | HGK_GRAVE),
];

pub(crate) const GREEK_UPPER: &[char] = &[
    '\u{0391}', '\u{0392}', '\u{03A8}', '\u{0394}', '\u{0395}', '\u{03A6}', '\u{0393}', '\u{0397}',
    '\u{0399}', '\u{039E}', '\u{039A}', '\u{039B}', '\u{039C}', '\u{039D}', '\u{039F}', '\u{03A0}',
    '\u{03DC}', '\u{03A1}', '\u{03A3}', '\u{03A4}', '\u{0398}', '\u{03A9}', '\u{00B7}', '\u{03A7}',
    '\u{03A5}', '\u{0396}',
];

pub(crate) const GREEK_LOWER: &[char] = &[
    '\u{03B1}', '\u{03B2}', '\u{03C8}', '\u{03B4}', '\u{03B5}', '\u{03C6}', '\u{03B3}', '\u{03B7}',
    '\u{03B9}', '\u{03BE}', '\u{03BA}', '\u{03BB}', '\u{03BC}', '\u{03BD}', '\u{03BF}', '\u{03C0}',
    '\u{03DD}', '\u{03C1}', '\u{03C3}', '\u{03C4}', '\u{03B8}', '\u{03C9}', '\u{03C2}', '\u{03C7}',
    '\u{03C5}', '\u{03B6}',
];

pub(crate) const GREEK_LOWER_PUA: &[char] = &[
    '\u{EB04}', //alpha
    '\u{EB07}', '\u{EAF3}', '\u{EB05}', '\u{EB09}', '\u{EAF4}', '\u{EB00}', '\u{EAF0}', '\u{EAF9}',
    '\u{EB0C}', '\u{EAFA}', '\u{EB0B}', '\u{EAFB}', '\u{EAFC}', '\u{EB0A}', '\u{EAF8}',
    '\u{EB3C}', //iota
    '\u{EB3D}', '\u{EB54}', '\u{EB3E}', '\u{EB3F}', '\u{EB55}', '\u{EB39}', '\u{EB38}', '\u{EB41}',
    '\u{EB42}', '\u{EB45}', '\u{EB43}', '\u{EB47}', '\u{EB48}', '\u{EB40}', '\u{EB44}',
    '\u{EB7D}', //upsilon
    '\u{EB7F}', '\u{EB71}', '\u{EB7E}', '\u{EB80}', '\u{EB75}', '\u{EB7A}', '\u{EB6F}', '\u{EB84}',
    '\u{EB85}', '\u{EB88}', '\u{EB82}', '\u{EB89}', '\u{EB8A}', '\u{EB81}', '\u{EB83}',
];

pub const NOT_ACCENTABLE_CHAR: char = '\u{0001}'; //dummy value
pub const NOCHAR: u32 = 0;

const HGK_NO_SORT: u32 = 0;
const HGK_ALPHA_SORT: u32 = 1;
const HGK_BETA_SORT: u32 = 2;
const HGK_GAMMA_SORT: u32 = 3;
const HGK_DELTA_SORT: u32 = 4;
const HGK_EPSILON_SORT: u32 = 5;
const HGK_DIGAMMA_SORT: u32 = 6;
const HGK_ZETA_SORT: u32 = 7;
const HGK_ETA_SORT: u32 = 8;
const HGK_THETA_SORT: u32 = 9;
const HGK_IOTA_SORT: u32 = 10;
const HGK_KAPPA_SORT: u32 = 11;
const HGK_LAMBDA_SORT: u32 = 12;
const HGK_MU_SORT: u32 = 13;
const HGK_NU_SORT: u32 = 14;
const HGK_XI_SORT: u32 = 15;
const HGK_OMICRON_SORT: u32 = 16;
const HGK_PI_SORT: u32 = 17;
const HGK_SAN_SORT: u32 = 18;
const HGK_KOPPA_SORT: u32 = 19;
const HGK_RHO_SORT: u32 = 20;
const HGK_SIGMA_SORT: u32 = 21;
const HGK_TAU_SORT: u32 = 22;
const HGK_UPSILON_SORT: u32 = 23;
const HGK_PHI_SORT: u32 = 24;
const HGK_CHI_SORT: u32 = 25;
const HGK_PSI_SORT: u32 = 26;
const HGK_OMEGA_SORT: u32 = 27;

//first col will be the actual codepoint for an accenting character
//or NOT_ACCENTABLE_CHAR or NOCHAR
pub(crate) const GREEK_BASIC: &[(char, u32, u32)] = &[
    /* 0370 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT), //capital heta
    /* 0371 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT), //lower case heta
    /* 0372 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT), //capital sampi
    /* 0373 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT), //lower case sampi
    /* 0374 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0375 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0376 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0377 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0378 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0379 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 037A */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 037B */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 037C */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 037D */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 037E */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 037F */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0380 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0381 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0382 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0383 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0384 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0385 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0386 */ ('\u{0391}', HGK_ACUTE, HGK_ALPHA_SORT),
    /* 0387 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 0388 */ ('\u{0395}', HGK_ACUTE, HGK_EPSILON_SORT),
    /* 0389 */ ('\u{0397}', HGK_ACUTE, HGK_ETA_SORT),
    /* 038A */ ('\u{0399}', HGK_ACUTE, HGK_IOTA_SORT),
    /* 038B */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 038C */ ('\u{039F}', HGK_ACUTE, HGK_OMICRON_SORT),
    /* 038D */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 038E */ ('\u{03A5}', HGK_ACUTE, HGK_UPSILON_SORT),
    /* 038F */ ('\u{03A9}', HGK_ACUTE, HGK_OMEGA_SORT),
    /* 0390 */ ('\u{03B9}', HGK_ACUTE | HGK_DIAERESIS, HGK_IOTA_SORT),
    /* 0391 */ ('\u{0391}', 0, HGK_ALPHA_SORT),
    /* 0392 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_BETA_SORT), /* capital beta */
    /* 0393 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_GAMMA_SORT), /* capital gamma */
    /* 0394 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_DELTA_SORT), /* capital delta */
    /* 0395 */ ('\u{0395}', 0, HGK_EPSILON_SORT),
    /* 0396 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_ZETA_SORT), /* capital zeta */
    /* 0397 */ ('\u{0397}', 0, HGK_ETA_SORT),
    /* 0398 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_THETA_SORT), /* capital theta */
    /* 0399 */ ('\u{0399}', 0, HGK_IOTA_SORT),
    /* 039A */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_KAPPA_SORT), /* capital kappa */
    /* 039B */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_LAMBDA_SORT), /* capital lambda */
    /* 039C */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_MU_SORT), /* capital mu */
    /* 039D */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_NU_SORT), /* capital nu */
    /* 039E */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_XI_SORT), /* capital xi */
    /* 039F */ ('\u{039F}', 0, HGK_OMICRON_SORT),
    /* 03A0 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_PI_SORT), /* capital pi */
    /* 03A1 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_RHO_SORT), /* capital rho */
    /* 03A2 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03A3 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_SIGMA_SORT), /* capital sigma */
    /* 03A4 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_TAU_SORT), /* capital tau */
    /* 03A5 */ ('\u{03A5}', 0, HGK_UPSILON_SORT),
    /* 03A6 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_PHI_SORT), /* capital phi */
    /* 03A7 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_CHI_SORT), /* capital chi */
    /* 03A8 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_PSI_SORT), /* capital psi */
    /* 03A9 */ ('\u{03A9}', 0, HGK_OMEGA_SORT),
    /* 03AA */ ('\u{0399}', HGK_DIAERESIS, HGK_IOTA_SORT),
    /* 03AB */ ('\u{03A5}', HGK_DIAERESIS, HGK_UPSILON_SORT),
    /* 03AC */ ('\u{03B1}', HGK_ACUTE, HGK_ALPHA_SORT),
    /* 03AD */ ('\u{03B5}', HGK_ACUTE, HGK_EPSILON_SORT),
    /* 03AE */ ('\u{03B7}', HGK_ACUTE, HGK_ETA_SORT),
    /* 03AF */ ('\u{03B9}', HGK_ACUTE, HGK_IOTA_SORT),
    /* 03B0 */ ('\u{03C5}', HGK_ACUTE | HGK_DIAERESIS, HGK_UPSILON_SORT),
    /* 03B1 */ ('\u{03B1}', 0, HGK_ALPHA_SORT),
    /* 03B2 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_BETA_SORT), /* small beta */
    /* 03B3 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_GAMMA_SORT), /* small gamma */
    /* 03B4 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_DELTA_SORT), /* small delta */
    /* 03B5 */ ('\u{03B5}', 0, HGK_EPSILON_SORT),
    /* 03B6 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_ZETA_SORT), /* small zeta */
    /* 03B7 */ ('\u{03B7}', 0, HGK_ETA_SORT),
    /* 03B8 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_THETA_SORT), /* small theta */
    /* 03B9 */ ('\u{03B9}', 0, HGK_IOTA_SORT),
    /* 03BA */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_KAPPA_SORT), /* small kappa */
    /* 03BB */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_LAMBDA_SORT), /* small lambda */
    /* 03BC */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_MU_SORT), /* small mu */
    /* 03BD */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_NU_SORT), /* small nu */
    /* 03BE */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_XI_SORT), /* small xi */
    /* 03BF */ ('\u{03BF}', 0, HGK_OMICRON_SORT),
    /* 03C0 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_PI_SORT), /* small pi */
    /* 03C1 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_RHO_SORT), /* small rho */
    /* 03C2 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_SIGMA_SORT), /* small final sigma */
    /* 03C3 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_SIGMA_SORT), /* small sigma */
    /* 03C4 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_TAU_SORT), /* small tau */
    /* 03C5 */ ('\u{03C5}', 0, HGK_UPSILON_SORT),
    /* 03C6 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_PHI_SORT), /* small phi */
    /* 03C7 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_CHI_SORT), /* small chi */
    /* 03C8 */ (NOT_ACCENTABLE_CHAR, NOCHAR, HGK_PSI_SORT), /* small psi */
    /* 03C9 */ ('\u{03C9}', 0, HGK_OMEGA_SORT),
    /* 03CA */ ('\u{03B9}', HGK_DIAERESIS, HGK_IOTA_SORT),
    /* 03CB */ ('\u{03C5}', HGK_DIAERESIS, HGK_UPSILON_SORT),
    /* 03CC */ ('\u{03BF}', HGK_ACUTE, HGK_OMICRON_SORT),
    /* 03CD */ ('\u{03C5}', HGK_ACUTE, HGK_UPSILON_SORT),
    /* 03CE */ ('\u{03C9}', HGK_ACUTE, HGK_OMEGA_SORT),
    /* 03CF */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03D0 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03D1 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03D2 */ ('\u{03D2}', HGK_NO_DIACRITICS, HGK_UPSILON_SORT), /* hook upsilon */
    /* 03D3 */ ('\u{03D2}', HGK_ACUTE, HGK_UPSILON_SORT), /* hook upsilon */
    /* 03D4 */ ('\u{03D2}', HGK_DIAERESIS, HGK_UPSILON_SORT), /* hook upsilon */
    /* 03D5 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03D6 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03D7 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03D8 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03D9 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03DA */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03DB */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03DC */ ('\u{03DC}', 0, HGK_DIGAMMA_SORT), /* DIGAMMA */
    /* 03DD */ ('\u{03DD}', 0, HGK_DIGAMMA_SORT), /* SMALL DIGAMMA */
    /* 03DE */ ('\u{03DE}', 0, HGK_KOPPA_SORT), /* KOPPA */
    /* 03DF */ ('\u{03DF}', 0, HGK_KOPPA_SORT), /* SMALL KOPPA */
    /* 03E0 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E1 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E2 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E3 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E4 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E5 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E6 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E7 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E8 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03E9 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03EA */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03EB */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03EC */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03ED */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03EE */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03EF */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F0 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F1 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F2 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F3 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F4 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F5 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F6 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F7 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F8 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03F9 */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03FA */ ('\u{03FA}', 0, HGK_SAN_SORT), /* CAPITAL SAN */
    /* 03FB */ ('\u{03FB}', 0, HGK_SAN_SORT), /* SMALL SAN */
    /* 03FC */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03FD */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03FE */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
    /* 03FF */ ('\u{0000}', HGK_NO_DIACRITICS, HGK_NO_SORT),
];

pub(crate) const GREEK_EXTENDED: &[(char, u32)] = &[
    /* 1F00 */ ('\u{03B1}', HGK_SMOOTH),
    /* 1F01 */ ('\u{03B1}', HGK_ROUGH),
    /* 1F02 */ ('\u{03B1}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F03 */ ('\u{03B1}', HGK_ROUGH | HGK_GRAVE),
    /* 1F04 */ ('\u{03B1}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F05 */ ('\u{03B1}', HGK_ROUGH | HGK_ACUTE),
    /* 1F06 */ ('\u{03B1}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F07 */ ('\u{03B1}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F08 */ ('\u{0391}', HGK_SMOOTH),
    /* 1F09 */ ('\u{0391}', HGK_ROUGH),
    /* 1F0A */ ('\u{0391}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F0B */ ('\u{0391}', HGK_ROUGH | HGK_GRAVE),
    /* 1F0C */ ('\u{0391}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F0D */ ('\u{0391}', HGK_ROUGH | HGK_ACUTE),
    /* 1F0E */ ('\u{0391}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F0F */ ('\u{0391}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F10 */ ('\u{03B5}', HGK_SMOOTH),
    /* 1F11 */ ('\u{03B5}', HGK_ROUGH),
    /* 1F12 */ ('\u{03B5}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F13 */ ('\u{03B5}', HGK_ROUGH | HGK_GRAVE),
    /* 1F14 */ ('\u{03B5}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F15 */ ('\u{03B5}', HGK_ROUGH | HGK_ACUTE),
    /* 1F16 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F17 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F18 */ ('\u{0395}', HGK_SMOOTH),
    /* 1F19 */ ('\u{0395}', HGK_ROUGH),
    /* 1F1A */ ('\u{0395}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F1B */ ('\u{0395}', HGK_ROUGH | HGK_GRAVE),
    /* 1F1C */ ('\u{0395}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F1D */ ('\u{0395}', HGK_ROUGH | HGK_ACUTE),
    /* 1F1E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F1F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F20 */ ('\u{03B7}', HGK_SMOOTH),
    /* 1F21 */ ('\u{03B7}', HGK_ROUGH),
    /* 1F22 */ ('\u{03B7}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F23 */ ('\u{03B7}', HGK_ROUGH | HGK_GRAVE),
    /* 1F24 */ ('\u{03B7}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F25 */ ('\u{03B7}', HGK_ROUGH | HGK_ACUTE),
    /* 1F26 */ ('\u{03B7}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F27 */ ('\u{03B7}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F28 */ ('\u{0397}', HGK_SMOOTH),
    /* 1F29 */ ('\u{0397}', HGK_ROUGH),
    /* 1F2A */ ('\u{0397}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F2B */ ('\u{0397}', HGK_ROUGH | HGK_GRAVE),
    /* 1F2C */ ('\u{0397}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F2D */ ('\u{0397}', HGK_ROUGH | HGK_ACUTE),
    /* 1F2E */ ('\u{0397}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F2F */ ('\u{0397}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F30 */ ('\u{03B9}', HGK_SMOOTH),
    /* 1F31 */ ('\u{03B9}', HGK_ROUGH),
    /* 1F32 */ ('\u{03B9}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F33 */ ('\u{03B9}', HGK_ROUGH | HGK_GRAVE),
    /* 1F34 */ ('\u{03B9}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F35 */ ('\u{03B9}', HGK_ROUGH | HGK_ACUTE),
    /* 1F36 */ ('\u{03B9}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F37 */ ('\u{03B9}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F38 */ ('\u{0399}', HGK_SMOOTH),
    /* 1F39 */ ('\u{0399}', HGK_ROUGH),
    /* 1F3A */ ('\u{0399}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F3B */ ('\u{0399}', HGK_ROUGH | HGK_GRAVE),
    /* 1F3C */ ('\u{0399}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F3D */ ('\u{0399}', HGK_ROUGH | HGK_ACUTE),
    /* 1F3E */ ('\u{0399}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F3F */ ('\u{0399}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F40 */ ('\u{03BF}', HGK_SMOOTH),
    /* 1F41 */ ('\u{03BF}', HGK_ROUGH),
    /* 1F42 */ ('\u{03BF}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F43 */ ('\u{03BF}', HGK_ROUGH | HGK_GRAVE),
    /* 1F44 */ ('\u{03BF}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F45 */ ('\u{03BF}', HGK_ROUGH | HGK_ACUTE),
    /* 1F46 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F47 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F48 */ ('\u{039F}', HGK_SMOOTH),
    /* 1F49 */ ('\u{039F}', HGK_ROUGH),
    /* 1F4A */ ('\u{039F}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F4B */ ('\u{039F}', HGK_ROUGH | HGK_GRAVE),
    /* 1F4C */ ('\u{039F}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F4D */ ('\u{039F}', HGK_ROUGH | HGK_ACUTE),
    /* 1F4E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F4F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F50 */ ('\u{03C5}', HGK_SMOOTH),
    /* 1F51 */ ('\u{03C5}', HGK_ROUGH),
    /* 1F52 */ ('\u{03C5}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F53 */ ('\u{03C5}', HGK_ROUGH | HGK_GRAVE),
    /* 1F54 */ ('\u{03C5}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F55 */ ('\u{03C5}', HGK_ROUGH | HGK_ACUTE),
    /* 1F56 */ ('\u{03C5}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F57 */ ('\u{03C5}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F58 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F59 */ ('\u{03A5}', HGK_ROUGH),
    /* 1F5A */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F5B */ ('\u{03A5}', HGK_ROUGH | HGK_GRAVE),
    /* 1F5C */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F5D */ ('\u{03A5}', HGK_ROUGH | HGK_ACUTE),
    /* 1F5E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F5F */ ('\u{03A5}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F60 */ ('\u{03C9}', HGK_SMOOTH),
    /* 1F61 */ ('\u{03C9}', HGK_ROUGH),
    /* 1F62 */ ('\u{03C9}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F63 */ ('\u{03C9}', HGK_ROUGH | HGK_GRAVE),
    /* 1F64 */ ('\u{03C9}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F65 */ ('\u{03C9}', HGK_ROUGH | HGK_ACUTE),
    /* 1F66 */ ('\u{03C9}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F67 */ ('\u{03C9}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F68 */ ('\u{03A9}', HGK_SMOOTH),
    /* 1F69 */ ('\u{03A9}', HGK_ROUGH),
    /* 1F6A */ ('\u{03A9}', HGK_SMOOTH | HGK_GRAVE),
    /* 1F6B */ ('\u{03A9}', HGK_ROUGH | HGK_GRAVE),
    /* 1F6C */ ('\u{03A9}', HGK_SMOOTH | HGK_ACUTE),
    /* 1F6D */ ('\u{03A9}', HGK_ROUGH | HGK_ACUTE),
    /* 1F6E */ ('\u{03A9}', HGK_SMOOTH | HGK_CIRCUMFLEX),
    /* 1F6F */ ('\u{03A9}', HGK_ROUGH | HGK_CIRCUMFLEX),
    /* 1F70 */ ('\u{03B1}', HGK_GRAVE),
    /* 1F71 */ ('\u{03B1}', HGK_ACUTE),
    /* 1F72 */ ('\u{03B5}', HGK_GRAVE),
    /* 1F73 */ ('\u{03B5}', HGK_ACUTE),
    /* 1F74 */ ('\u{03B7}', HGK_GRAVE),
    /* 1F75 */ ('\u{03B7}', HGK_ACUTE),
    /* 1F76 */ ('\u{03B9}', HGK_GRAVE),
    /* 1F77 */ ('\u{03B9}', HGK_ACUTE),
    /* 1F78 */ ('\u{03BF}', HGK_GRAVE),
    /* 1F79 */ ('\u{03BF}', HGK_ACUTE),
    /* 1F7A */ ('\u{03C5}', HGK_GRAVE),
    /* 1F7B */ ('\u{03C5}', HGK_ACUTE),
    /* 1F7C */ ('\u{03C9}', HGK_GRAVE),
    /* 1F7D */ ('\u{03C9}', HGK_ACUTE),
    /* 1F7E */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F7F */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1F80 */ ('\u{03B1}', HGK_SMOOTH | HGK_IOTA_SUBSCRIPT),
    /* 1F81 */ ('\u{03B1}', HGK_ROUGH | HGK_IOTA_SUBSCRIPT),
    /* 1F82 */ ('\u{03B1}', HGK_SMOOTH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F83 */ ('\u{03B1}', HGK_ROUGH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F84 */ ('\u{03B1}', HGK_SMOOTH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F85 */ ('\u{03B1}', HGK_ROUGH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F86 */ ('\u{03B1}', HGK_SMOOTH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1F87 */ ('\u{03B1}', HGK_ROUGH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1F88 */ ('\u{0391}', HGK_SMOOTH | HGK_IOTA_SUBSCRIPT),
    /* 1F89 */ ('\u{0391}', HGK_ROUGH | HGK_IOTA_SUBSCRIPT),
    /* 1F8A */ ('\u{0391}', HGK_SMOOTH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F8B */ ('\u{0391}', HGK_ROUGH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F8C */ ('\u{0391}', HGK_SMOOTH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F8D */ ('\u{0391}', HGK_ROUGH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F8E */ ('\u{0391}', HGK_SMOOTH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1F8F */ ('\u{0391}', HGK_ROUGH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1F90 */ ('\u{03B7}', HGK_SMOOTH | HGK_IOTA_SUBSCRIPT),
    /* 1F91 */ ('\u{03B7}', HGK_ROUGH | HGK_IOTA_SUBSCRIPT),
    /* 1F92 */ ('\u{03B7}', HGK_SMOOTH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F93 */ ('\u{03B7}', HGK_ROUGH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F94 */ ('\u{03B7}', HGK_SMOOTH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F95 */ ('\u{03B7}', HGK_ROUGH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F96 */ ('\u{03B7}', HGK_SMOOTH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1F97 */ ('\u{03B7}', HGK_ROUGH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1F98 */ ('\u{0397}', HGK_SMOOTH | HGK_IOTA_SUBSCRIPT),
    /* 1F99 */ ('\u{0397}', HGK_ROUGH | HGK_IOTA_SUBSCRIPT),
    /* 1F9A */ ('\u{0397}', HGK_SMOOTH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F9B */ ('\u{0397}', HGK_ROUGH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1F9C */ ('\u{0397}', HGK_SMOOTH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F9D */ ('\u{0397}', HGK_ROUGH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1F9E */ ('\u{0397}', HGK_SMOOTH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1F9F */ ('\u{0397}', HGK_ROUGH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FA0 */ ('\u{03C9}', HGK_SMOOTH | HGK_IOTA_SUBSCRIPT),
    /* 1FA1 */ ('\u{03C9}', HGK_ROUGH | HGK_IOTA_SUBSCRIPT),
    /* 1FA2 */ ('\u{03C9}', HGK_SMOOTH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1FA3 */ ('\u{03C9}', HGK_ROUGH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1FA4 */ ('\u{03C9}', HGK_SMOOTH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1FA5 */ ('\u{03C9}', HGK_ROUGH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1FA6 */ ('\u{03C9}', HGK_SMOOTH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FA7 */ ('\u{03C9}', HGK_ROUGH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FA8 */ ('\u{03A9}', HGK_SMOOTH | HGK_IOTA_SUBSCRIPT),
    /* 1FA9 */ ('\u{03A9}', HGK_ROUGH | HGK_IOTA_SUBSCRIPT),
    /* 1FAA */ ('\u{03A9}', HGK_SMOOTH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1FAB */ ('\u{03A9}', HGK_ROUGH | HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1FAC */ ('\u{03A9}', HGK_SMOOTH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1FAD */ ('\u{03A9}', HGK_ROUGH | HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1FAE */ ('\u{03A9}', HGK_SMOOTH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FAF */ ('\u{03A9}', HGK_ROUGH | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FB0 */ ('\u{03B1}', HGK_BREVE), //new
    /* 1FB1 */ ('\u{03B1}', HGK_MACRON),
    /* 1FB2 */ ('\u{03B1}', HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1FB3 */ ('\u{03B1}', HGK_IOTA_SUBSCRIPT),
    /* 1FB4 */ ('\u{03B1}', HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1FB5 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FB6 */ ('\u{03B1}', HGK_CIRCUMFLEX),
    /* 1FB7 */ ('\u{03B1}', HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FB8 */ ('\u{0391}', HGK_BREVE),
    /* 1FB9 */ ('\u{0391}', HGK_MACRON),
    /* 1FBA */ ('\u{0391}', HGK_GRAVE),
    /* 1FBB */ ('\u{0391}', HGK_ACUTE),
    /* 1FBC */ ('\u{0391}', HGK_IOTA_SUBSCRIPT),
    /* 1FBD */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FBE */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FBF */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FC0 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FC1 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FC2 */ ('\u{03B7}', HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1FC3 */ ('\u{03B7}', HGK_IOTA_SUBSCRIPT),
    /* 1FC4 */ ('\u{03B7}', HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1FC5 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FC6 */ ('\u{03B7}', HGK_CIRCUMFLEX),
    /* 1FC7 */ ('\u{03B7}', HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FC8 */ ('\u{0395}', HGK_GRAVE),
    /* 1FC9 */ ('\u{0395}', HGK_ACUTE),
    /* 1FCA */ ('\u{0397}', HGK_GRAVE),
    /* 1FCB */ ('\u{0397}', HGK_ACUTE),
    /* 1FCC */ ('\u{0397}', HGK_IOTA_SUBSCRIPT),
    /* 1FCD */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FCE */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FCF */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FD0 */ ('\u{03B9}', HGK_BREVE), //new
    /* 1FD1 */ ('\u{03B9}', HGK_MACRON),
    /* 1FD2 */ ('\u{03B9}', HGK_DIAERESIS | HGK_GRAVE),
    /* 1FD3 */ ('\u{03B9}', HGK_DIAERESIS | HGK_ACUTE),
    /* 1FD4 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FD5 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FD6 */ ('\u{03B9}', HGK_CIRCUMFLEX),
    /* 1FD7 */ ('\u{03B9}', HGK_DIAERESIS | HGK_CIRCUMFLEX),
    /* 1FD8 */ ('\u{0399}', HGK_BREVE),
    /* 1FD9 */ ('\u{0399}', HGK_MACRON),
    /* 1FDA */ ('\u{0399}', HGK_GRAVE),
    /* 1FDB */ ('\u{0399}', HGK_ACUTE),
    /* 1FDC */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FDD */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FDE */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FDF */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FE0 */ ('\u{03C5}', HGK_BREVE), //new
    /* 1FE1 */ ('\u{03C5}', HGK_MACRON),
    /* 1FE2 */ ('\u{03C5}', HGK_DIAERESIS | HGK_GRAVE),
    /* 1FE3 */ ('\u{03C5}', HGK_DIAERESIS | HGK_ACUTE),
    /* 1FE4 */ ('\u{03C1}', HGK_SMOOTH),
    /* 1FE5 */ ('\u{03C1}', HGK_ROUGH),
    /* 1FE6 */ ('\u{03C5}', HGK_CIRCUMFLEX),
    /* 1FE7 */ ('\u{03C5}', HGK_DIAERESIS | HGK_CIRCUMFLEX),
    /* 1FE8 */ ('\u{03A5}', HGK_BREVE),
    /* 1FE9 */ ('\u{03A5}', HGK_MACRON),
    /* 1FEA */ ('\u{03A5}', HGK_GRAVE),
    /* 1FEB */ ('\u{03A5}', HGK_ACUTE),
    /* 1FEC */ ('\u{03A1}', HGK_ROUGH),
    /* 1FED */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FEE */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FEF */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FF0 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FF1 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FF2 */ ('\u{03C9}', HGK_GRAVE | HGK_IOTA_SUBSCRIPT),
    /* 1FF3 */ ('\u{03C9}', HGK_IOTA_SUBSCRIPT),
    /* 1FF4 */ ('\u{03C9}', HGK_ACUTE | HGK_IOTA_SUBSCRIPT),
    /* 1FF5 */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FF6 */ ('\u{03C9}', HGK_CIRCUMFLEX),
    /* 1FF7 */ ('\u{03C9}', HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT),
    /* 1FF8 */ ('\u{039F}', HGK_GRAVE),
    /* 1FF9 */ ('\u{039F}', HGK_ACUTE),
    /* 1FFA */ ('\u{03A9}', HGK_GRAVE),
    /* 1FFB */ ('\u{03A9}', HGK_ACUTE),
    /* 1FFC */ ('\u{03A9}', HGK_IOTA_SUBSCRIPT),
    /* 1FFD */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FFE */ ('\u{0000}', HGK_NO_DIACRITICS),
    /* 1FFF */ ('\u{0000}', HGK_NO_DIACRITICS),
];

#[cfg(not(feature = "unicode-normalization"))]
pub(crate) const GREEK_PRECOMPOSED: [[char; 32]; 14] = [
    /*
    base, tonos, dia+tonos, psili, dasia, oxia, psi+oxia,
    dasi+oxia, varia, psi+varia, das+varia, perisp, psi+peri, das+peri,
    upo, psi+upo, das+upo, ox+upo, psi+ox+upo, das+ox+upo, var+upo,
    psi+var+upo, das+var+upo, per+upo, si+per+upo, das+per+upo, dialytika, dia+ox,
    dia+var, dia+peri, macron, vrachy
    */

    /* alpha */
    [
        '\u{03B1}', '\u{03AC}', '\u{0000}', '\u{1F00}', '\u{1F01}', '\u{1F71}', '\u{1F04}',
        '\u{1F05}', '\u{1F70}', '\u{1F02}', '\u{1F03}', '\u{1FB6}', '\u{1F06}', '\u{1F07}',
        '\u{1FB3}', '\u{1F80}', '\u{1F81}', '\u{1FB4}', '\u{1F84}', '\u{1F85}', '\u{1FB2}',
        '\u{1F82}', '\u{1F83}', '\u{1FB7}', '\u{1F86}', '\u{1F87}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{1FB1}', '\u{1FB0}',
    ],
    /* epsilon */
    [
        '\u{03B5}', '\u{03AD}', '\u{0000}', '\u{1F10}', '\u{1F11}', '\u{1F73}', '\u{1F14}',
        '\u{1F15}', '\u{1F72}', '\u{1F12}', '\u{1F13}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
    ],
    /* eta */
    [
        '\u{03B7}', '\u{03AE}', '\u{0000}', '\u{1F20}', '\u{1F21}', '\u{1F75}', '\u{1F24}',
        '\u{1F25}', '\u{1F74}', '\u{1F22}', '\u{1F23}', '\u{1FC6}', '\u{1F26}', '\u{1F27}',
        '\u{1FC3}', '\u{1F90}', '\u{1F91}', '\u{1FC4}', '\u{1F94}', '\u{1F95}', '\u{1FC2}',
        '\u{1F92}', '\u{1F93}', '\u{1FC7}', '\u{1F96}', '\u{1F97}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{1FD0}',
    ],
    /* iota */
    [
        '\u{03B9}', '\u{03AF}', '\u{0390}', '\u{1F30}', '\u{1F31}', '\u{1F77}', '\u{1F34}',
        '\u{1F35}', '\u{1F76}', '\u{1F32}', '\u{1F33}', '\u{1FD6}', '\u{1F36}', '\u{1F37}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{03CA}', '\u{1FD3}',
        '\u{1FD2}', '\u{1FD7}', '\u{1FD1}', '\u{0000}',
    ],
    /* omicron */
    [
        '\u{03BF}', '\u{03CC}', '\u{0000}', '\u{1F40}', '\u{1F41}', '\u{1F79}', '\u{1F44}',
        '\u{1F45}', '\u{1F78}', '\u{1F42}', '\u{1F43}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
    ],
    /* upsilon */
    [
        '\u{03C5}', '\u{03CD}', '\u{03B0}', '\u{1F50}', '\u{1F51}', '\u{1F7B}', '\u{1F54}',
        '\u{1F55}', '\u{1F7A}', '\u{1F52}', '\u{1F53}', '\u{1FE6}', '\u{1F56}', '\u{1F57}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{03CB}', '\u{1FE3}',
        '\u{1FE2}', '\u{1FE7}', '\u{1FE1}', '\u{1FE0}',
    ],
    /* omega */
    [
        '\u{03C9}', '\u{03CE}', '\u{0000}', '\u{1F60}', '\u{1F61}', '\u{1F7D}', '\u{1F64}',
        '\u{1F65}', '\u{1F7C}', '\u{1F62}', '\u{1F63}', '\u{1FF6}', '\u{1F66}', '\u{1F67}',
        '\u{1FF3}', '\u{1FA0}', '\u{1FA1}', '\u{1FF4}', '\u{1FA4}', '\u{1FA5}', '\u{1FF2}',
        '\u{1FA2}', '\u{1FA3}', '\u{1FF7}', '\u{1FA6}', '\u{1FA7}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
    ],
    /* capitals */
    /* alpha */
    [
        '\u{0391}', '\u{0386}', '\u{0000}', '\u{1F08}', '\u{1F09}', '\u{1FBB}', '\u{1F0C}',
        '\u{1F0D}', '\u{1FBA}', '\u{1F0A}', '\u{1F0B}', '\u{0000}', '\u{1F0E}', '\u{1F0F}',
        '\u{0000}', '\u{1F88}', '\u{1F89}', '\u{0000}', '\u{1F8C}', '\u{1F8D}', '\u{0000}',
        '\u{1F8A}', '\u{1F8B}', '\u{0000}', '\u{1F8E}', '\u{1F8F}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{1FB9}', '\u{1FB8}',
    ],
    /* epsilon */
    [
        '\u{0395}', '\u{0388}', '\u{0000}', '\u{1F18}', '\u{1F19}', '\u{1FC9}', '\u{1F1C}',
        '\u{1F1D}', '\u{1FC8}', '\u{1F1A}', '\u{1F1B}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
    ],
    /* eta */
    [
        '\u{0397}', '\u{0389}', '\u{0000}', '\u{1F28}', '\u{1F29}', '\u{1FCB}', '\u{1F2C}',
        '\u{1F2D}', '\u{1FCA}', '\u{1F2A}', '\u{1F2B}', '\u{0000}', '\u{1F2E}', '\u{1F2F}',
        '\u{0000}', '\u{1F98}', '\u{1F99}', '\u{0000}', '\u{1F9C}', '\u{1F9D}', '\u{0000}',
        '\u{1F9A}', '\u{1F9B}', '\u{0000}', '\u{1F9E}', '\u{1F9F}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
    ],
    /* iota */
    [
        '\u{0399}', '\u{038A}', '\u{0000}', '\u{1F38}', '\u{1F39}', '\u{1FDB}', '\u{1F3C}',
        '\u{1F3D}', '\u{1FDA}', '\u{1F3A}', '\u{1F3B}', '\u{0000}', '\u{1F3E}', '\u{1F3F}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{03AA}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{1FD9}', '\u{1FD8}',
    ],
    /* omicron */
    [
        '\u{039F}', '\u{038C}', '\u{0000}', '\u{1F48}', '\u{1F49}', '\u{1FF9}', '\u{1F4C}',
        '\u{1F4D}', '\u{1FF8}', '\u{1F4A}', '\u{1F4B}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
    ],
    /* upsilon */
    [
        '\u{03A5}', '\u{038E}', '\u{0000}', '\u{0000}', '\u{1F59}', '\u{1FEB}', '\u{0000}',
        '\u{1F5D}', '\u{1FEA}', '\u{0000}', '\u{1F5B}', '\u{0000}', '\u{0000}', '\u{1F5F}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{03AB}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{1FE9}', '\u{1FE8}',
    ],
    /* omega */
    [
        '\u{03A9}', '\u{038F}', '\u{0000}', '\u{1F68}', '\u{1F69}', '\u{1FFB}', '\u{1F6C}',
        '\u{1F6D}', '\u{1FFA}', '\u{1F6A}', '\u{1F6B}', '\u{0000}', '\u{1F6E}', '\u{1F6F}',
        '\u{0000}', '\u{1FA8}', '\u{1FA9}', '\u{0000}', '\u{1FAC}', '\u{1FAD}', '\u{0000}',
        '\u{1FAA}', '\u{1FAB}', '\u{0000}', '\u{1FAE}', '\u{AFAF}', '\u{0000}', '\u{0000}',
        '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}',
    ],
];
