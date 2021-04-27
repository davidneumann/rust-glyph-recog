pub struct GlyphRays{
    pub width:u16,
    pub height:u8,
    pub pixels_from_top:i8,
    pub l2r: Vec<u16>,
    pub t2b: Vec<u16>,
    pub r2l: Vec<u16>,
    pub b2t: Vec<u16>,
    pub m2l: Vec<u16>,
    pub m2t: Vec<u16>,
    pub m2r: Vec<u16>,
    pub m2b: Vec<u16>
}
