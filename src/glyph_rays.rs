use std::fs::File;
use std::io;
use std::cmp;
use io::Read;

pub struct GlyphRays{
    pub width:u16,
    pub height:u8,
    pub pixels_from_top:i8,
    pub l2r: Vec<u16>,
    pub t2b: Vec<u8>,
    pub r2l: Vec<u16>,
    pub b2t: Vec<u8>,
    pub m2l: Vec<u16>,
    pub m2t: Vec<u8>,
    pub m2r: Vec<u16>,
    pub m2b: Vec<u8>,
    pub raw: Vec<bool>,
}

impl GlyphRays {
    pub fn from_file(input: &str) -> GlyphRays {
        //println!("Trying to open {}", input);
        let mut fin = File::open(input).unwrap();

        let mut buffer = [0; 2];
        fin.read(&mut buffer).unwrap();
        let width = u16::from_le_bytes(buffer);
        let mut buffer = [0; 1];
        fin.read(&mut buffer).unwrap();
        let height = u8::from_le_bytes(buffer);
        fin.read(&mut buffer).unwrap();
        let pixels_from_top = u8::from_le_bytes(buffer);
        // println!("{},{}", width, height);

        let mut buffer = [0; 100];
        let mut input:Vec<bool> = Vec::new();
        let mut count = 0;
        let len = width * (height as u16);
        loop {
            let read = fin.read(&mut buffer).unwrap();
            if read == 0 {
                break;
            }

            input.extend(buffer.iter().take(read).clone().flat_map(|item| {
                let mut splits = Vec::new();
                for i in 0..8{
                    let pixel = (item & (1 << 7 - i)) != 0;
                    splits.push(pixel);
                    count += 1;
                    if count >= len { break; }
                }
                splits.into_iter()
            }));
        }

        glyph_with_raw(width, height, pixels_from_top as i8, input)
    }
}

fn glyph_with_raw(width:u16, height:u8, pixels_from_top:i8, input:Vec<bool>) -> GlyphRays {
    let mut ray = GlyphRays {
        l2r: vec![width; height as usize],
        t2b: vec![height; width as usize],
        r2l: vec![width; height as usize],
        b2t: vec![height; width as usize],
        m2l: vec![width / 2; height as usize],
        m2t: vec![height / 2; width as usize],
        m2r: vec![width / 2; height as usize],
        m2b: vec![height / 2; width as usize],
        width,
        height,
        pixels_from_top,
        raw: input,
    };

    for i in 0..ray.raw.len() {
        let x = (i % (width as usize)) as u16;
        let y = (i / (width as usize)) as u8;
        let pixel = ray.raw[i as usize];
        if pixel {
            ray.l2r[(y as usize)] = cmp::min(x, ray.l2r[(y as usize)]);
            ray.r2l[(y as usize)] = cmp::min(width - x - 1, ray.r2l[(y as usize)]);
            ray.t2b[(x as usize)] = cmp::min(y, ray.t2b[(x as usize)]);
            ray.b2t[(x as usize)] = cmp::min(height - y - 1, ray.b2t[(x as usize)]);
            if x <  width  / 2 { ray.m2l[(y as usize)] = cmp::min(width / 2 - x, ray.m2l[(y as usize)]); }
            if x >=  width  / 2 { ray.m2r[(y as usize)] = cmp::min(x - width / 2, ray.m2r[(y as usize)]); }
            if y <  height / 2 { ray.m2t[(x as usize)] = cmp::min(height / 2 - y, ray.m2t[(x as usize)]); }
            if y >=  height / 2 { ray.m2b[(x as usize)] = cmp::min(y - height / 2, ray.m2b[(x as usize)]); }
        }
    }
    ray
}

// impl GlyphRays {
//     pub fn GetSubGlyph(&self, start_x:u16, width:u16) -> GlyphRays {
//         let mut min_t2b = 0u8;
//         let mut min_b2t = 0u8;
//         for i in start_x..start_x + width {
//             let i = i as usize;
//             min_t2b = cmp::min(min_t2b, self.t2b[i]);
//             min_b2t = cmp::min(min_b2t, self.b2t[i]);
//         }
//         let height = self.height - min_t2b - min_b2t;
//         let pixels_from_top = self.pixels_from_top + min_t2b as i8;
//         GlyphRays{
//             width,
//             height,
//             pixels_from_top,
//             l2r: (),
//             t2b: (),
//             r2l: (),
//             b2t: (),
//             m2l: (),
//             m2t: (),
//             m2r: (),
//             m2b: (),
//             raw: (),
//         }
//     }
// }
