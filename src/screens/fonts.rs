
static ROBOTO_BYTES: &[u8] = include_bytes!("../../resources/fonts/Roboto-Bold.ttf");
lazy_static![
    pub static ref ROBOTO: fontdue::Font = fontdue::Font::from_bytes(ROBOTO_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];

static ELFBOY_BYTES: &[u8] = include_bytes!("../../resources/fonts/Elfboyclassic.ttf");
lazy_static![
    pub static ref ELFBOY: fontdue::Font = fontdue::Font::from_bytes(ELFBOY_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];

static RETROSIGNATURE_BYTES: &[u8] = include_bytes!("../../resources/fonts/RetroSignature.otf");
lazy_static![
    pub static ref RETROSIGNATURE: fontdue::Font = fontdue::Font::from_bytes(RETROSIGNATURE_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];

static ROADRAGE_BYTES: &[u8] = include_bytes!("../../resources/fonts/Roadrage.otf");
lazy_static![
    pub static ref ROADRAGE: fontdue::Font = fontdue::Font::from_bytes(ROADRAGE_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];


static PIXELLARI_BYTES: &[u8] = include_bytes!("../../resources/fonts/Pixellari.ttf");
lazy_static![
    /// Use with multiples of 8px
    pub static ref PIXELLARI: fontdue::Font = fontdue::Font::from_bytes(PIXELLARI_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];

static PIXELOID_BYTES: &[u8] = include_bytes!("../../resources/fonts/pixeloid/PixeloidMono.ttf");
lazy_static![
    /// Use with multiples of 9px
    pub static ref PIXELOID: fontdue::Font = fontdue::Font::from_bytes(PIXELOID_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];

static SYMTEXT_BYTES: &[u8] = include_bytes!("../../resources/fonts/Symtext.ttf");
lazy_static![
    /// Use with multiples of 5px
    pub static ref SYMTEXT: fontdue::Font = fontdue::Font::from_bytes(SYMTEXT_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];

static SQHEAD_BYTES: &[u8] = include_bytes!("../../resources/fonts/SF Square Head.ttf");
lazy_static![
    /// Use with multiples of 8px
    pub static ref SQHEAD: fontdue::Font = fontdue::Font::from_bytes(SQHEAD_BYTES, fontdue::FontSettings::default()).expect("Failed to load font");
];
