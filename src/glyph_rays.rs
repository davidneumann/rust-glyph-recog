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
    pub raw: Vec<Vec<bool>>,
}

impl GlyphRays {
    pub fn from_read<T: Read>(fin: &mut T) -> GlyphRays {
        let mut buffer = [0; 2];
        fin.read(&mut buffer).unwrap();
        let width = u16::from_le_bytes(buffer);
        let mut buffer = [0; 1];
        fin.read(&mut buffer).unwrap();
        let height = u8::from_le_bytes(buffer);
        fin.read(&mut buffer).unwrap();
        let pixels_from_top = u8::from_le_bytes(buffer);
        // println!("{},{}", width, height);

        let mut count = 0;
        let len = (width * (height as u16)) as usize;
        let mut buffer = vec![0u8; len];
        let mut input = vec![vec![false; height as usize]; width as usize];
        let read = fin.read(&mut buffer).unwrap();

        for buffer_i in 0..read {
            let packed_bytes = buffer[buffer_i];
            for i in 0..8{
                let pixel = (packed_bytes & (1 << 7 - i)) != 0;
                input[count % width as usize][count / width as usize] = pixel;
                //splits.push(pixel);
                count += 1;
                if count >= len { break; }
            }
        }

        glyph_with_raw(width, height, pixels_from_top as i8, input)
    }

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
        let mut count = 0;
        let len = (width * (height as u16)) as usize;
        let mut input = vec![vec![false; height as usize]; width as usize];
        loop {
            let read = fin.read(&mut buffer).unwrap();
            if read == 0 {
                break;
            }

            for buffer_i in 0..read {
                let packed_bytes = buffer[buffer_i];
                for i in 0..8{
                    let pixel = (packed_bytes & (1 << 7 - i)) != 0;
                    input[count % width as usize][count / width as usize] = pixel;
                    //splits.push(pixel);
                    count += 1;
                    if count >= len { break; }
                }
            }
        }

        glyph_with_raw(width, height, pixels_from_top as i8, input)
    }

    pub fn empty() -> GlyphRays {
        GlyphRays {
            width: 0,
            height: 0,
            pixels_from_top: 0,
            l2r: Vec::new(),
            t2b: Vec::new(),
            r2l: Vec::new(),
            b2t: Vec::new(),
            m2l: Vec::new(),
            m2t: Vec::new(),
            m2r: Vec::new(),
            m2b: Vec::new(),
            raw: Vec::new(),
        }
    }
}

fn glyph_with_raw(width:u16, height:u8, pixels_from_top:i8, input:Vec<Vec<bool>>) -> GlyphRays {
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

    for y in 0..height as usize {
        for x in 0..width as usize {
            let pixel = ray.raw[x][y];
            let x = x as u16;
            let y = y as u8;
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
    }
    ray
}

impl GlyphRays {
    pub fn get_sub_glyph(&self, mut start_x:u16, mut width:u16) -> Option<GlyphRays> {
        let end_x = start_x + width;
        //Trim any leftmost columns that are empty
        for i in (start_x..start_x + width).rev() {
            if self.t2b[i as usize] < self.height {
                start_x = i as u16;
            }
        }

        let mut min_t2b = std::u8::MAX;
        let mut min_b2t = std::u8::MAX;
        for i in start_x..end_x {
            let i = i as usize;
            min_t2b = cmp::min(min_t2b, self.t2b[i]);
            min_b2t = cmp::min(min_b2t, self.b2t[i]);
        }
        if min_t2b >= self.height || min_b2t >= self.height { return None; }
        let height = self.height - min_t2b - min_b2t;
        let pixels_from_top = self.pixels_from_top + min_t2b as i8;

        let mut input = vec![vec![false; height as usize]; width as usize];
        let start_y = min_t2b as usize;
        let start_x = start_x as usize;
        width = end_x - start_x as u16;
        for y in 0..height as usize {
            for x in 0..width as usize {
                input[x][y] = self.raw[start_x + x][start_y + y];
            }
        }

        Some(glyph_with_raw(width, height, pixels_from_top, input))
    }
}

impl GlyphRays {
    pub fn average(samples:&Vec<GlyphRays>) -> GlyphRays {
        let mut total_raw = vec![vec![0usize; samples[0].height as usize]; samples[0].width as usize];
        let mut total_width = 0u32;
        let mut total_height = 0u32;
        let mut total_pixels_from_top = 0i32;
        let mut total_l2r = vec![0u32; samples[0].l2r.len()];
        let mut total_t2b = vec![0u32; samples[0].t2b.len()];
        let mut total_r2l = vec![0u32; samples[0].r2l.len()];
        let mut total_b2t = vec![0u32; samples[0].b2t.len()];
        let mut total_m2l = vec![0u32; samples[0].m2l.len()];
        let mut total_m2t = vec![0u32; samples[0].m2t.len()];
        let mut total_m2r = vec![0u32; samples[0].m2r.len()];
        let mut total_m2b = vec![0u32; samples[0].m2b.len()];
        for cur in samples {
            total_width += cur.width as u32;
            total_height += cur.height as u32;
            total_pixels_from_top += cur.pixels_from_top as i32;
            for x in 0..cur.width as usize {
                total_t2b[x] += cur.t2b[x] as u32;
                total_b2t[x] += cur.b2t[x] as u32;
                total_m2b[x] += cur.m2b[x] as u32;
                total_m2t[x] += cur.m2t[x] as u32;
            }
            for y in 0..cur.height as usize {
                total_l2r[y] += cur.l2r[y] as u32;
                total_r2l[y] += cur.r2l[y] as u32;
                total_m2r[y] += cur.m2r[y] as u32;
                total_m2l[y] += cur.m2l[y] as u32;
            }
            for x in 0..cur.width as usize {
                for y in 0..cur.height as usize {
                    if cur.raw[x][y] { total_raw[x][y] += 1; }
                }
            }
        }
        let count = samples.len();
        let mut avg_raw = vec![vec![false; samples[0].height as usize]; samples[0].width as usize];
        for x in 0..samples[0].width as usize {
            for y in 0..samples[0].height as usize {
                avg_raw[x][y] = total_raw[x][y]  >= count / 2 as usize
            }
        }
        GlyphRays {
            width: (total_width / count as u32) as u16,
            height: (total_height / count as u32) as u8,
            pixels_from_top: (total_pixels_from_top / count as i32) as i8,
            l2r: total_l2r.iter().map(|x| (x / count as u32) as u16).collect(),
            t2b: total_t2b.iter().map(|x| (x / count as u32) as u8).collect(),
            r2l: total_r2l.iter().map(|x| (x / count as u32) as u16).collect(),
            b2t: total_b2t.iter().map(|x| (x / count as u32) as u8).collect(),
            m2l: total_m2l.iter().map(|x| (x / count as u32) as u16).collect(),
            m2t: total_m2t.iter().map(|x| (x / count as u32) as u8).collect(),
            m2r: total_m2r.iter().map(|x| (x / count as u32) as u16).collect(),
            m2b: total_m2b.iter().map(|x| (x / count as u32) as u8).collect(),
            raw: avg_raw,
        }
    }
}

impl GlyphRays {
    pub fn print(&self) -> () {
        println!();
        println!("l2r {:?}", self.l2r);
        println!("r2l {:?}", self.r2l);
        println!("t2b {:?}", self.t2b);
        println!("b2t {:?}", self.b2t);
        println!("m2l {:?}", self.m2l);
        println!("m2r {:?}", self.m2r);
        println!("m2t {:?}", self.m2t);
        println!("m2b {:?}", self.m2b);
        println!("width {:?} height {:?} top {}", self.width, self.height, self.pixels_from_top);

        let height = self.height;
        for y in 0..height {
            for x in 0..self.width {
                if x < self.width - 1 &&  self.l2r[(y as usize)] == x      { print!("X"); }
                else if x > 0 && self.r2l[y as usize] < self.width && self.width - 1 - self.r2l[(y as usize)] == x { print!("X"); }
                else if y < height - 1 && self.t2b[(x as usize)] == y { print!("X"); }
                else if y > 0 && self.b2t[(x as usize)] < height && height - 1 - self.b2t[(x as usize)] == y { print!("X"); }
                else if x > 0 && self.width / 2 - self.m2l[(y as usize)] == x { print!("X"); }
                else if x > 0 && self.m2r[(y as usize)] + self.width / 2 == x { print!("X"); }
                else if y > 0 && height / 2 - self.m2t[(x as usize)] == y { print!("X"); }
                else if y < height - 1 && self.m2b[(x as usize)] + height / 2 == y { print!("X"); }
                else { print!(" "); }
            }
            println!();
        }
    }

    pub fn print_raw(&self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if self.raw[x][y] { print!("H"); }
                else { print!(" "); }
            }
            println!();
        }
    }
}
