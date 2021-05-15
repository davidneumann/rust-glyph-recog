use crate::glyph_rays::GlyphRays;

pub struct Glyph {
    pub value: String,
    pub max_error: u32,
    pub ray: GlyphRays,
}
