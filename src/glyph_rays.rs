use std::fs::File;
use std::io;
use std::cmp;
use io::Read;

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
    pub m2b: Vec<u16>,
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
        let height = u8::from_le_bytes(buffer) as u16;
        fin.read(&mut buffer).unwrap();
        let pixels_from_top = u8::from_le_bytes(buffer);
        // println!("{},{}", width, height);

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
            height: height as u8,
            pixels_from_top: pixels_from_top as i8,
            raw: vec![false; (width & height) as usize],
        };

        //    let mut l2r = vec![width; height as usize];

        let mut buffer = [0; 1];

        let mut count = 0;
        loop {
            let read = fin.read(&mut buffer).unwrap();
            if read == 0 {
                break;
            }

            for i in 0..8{
                let x = count % width;
                let y = count / width;
                let pixel = (buffer[0] & (1 << 7 - i)) != 0;
                ray.raw[count as usize] = pixel;
                if pixel {
                    ray.l2r[(y as usize)] = cmp::min(x, ray.l2r[(y as usize)]);
                    ray.r2l[(y as usize)] = cmp::min(width - x - 1, ray.r2l[(y as usize)]);
                    ray.t2b[(x as usize)] = cmp::min(y, ray.t2b[(x as usize)]);
                    ray.b2t[(x as usize)] = cmp::min(height - y - 1, ray.b2t[(x as usize)]);
                    if x <  width  / 2 { ray.m2l[(y as usize)] = cmp::min(width / 2 - x, ray.m2l[(y as usize)]); }
                    if x >=  width  / 2 { ray.m2r[(y as usize)] = cmp::min(x - width / 2, ray.m2r[(y as usize)]); }
                    if y <  height / 2 { ray.m2t[(x as usize)] = cmp::min(height / 2 - y, ray.m2t[(x as usize)]); }
                    if y >=  height / 2 { ray.m2b[(x as usize)] = cmp::min(y - height / 2, ray.m2b[(x as usize)]); }
                    // print!("X");
                }
                //else { print! (" ");}
                //println!("x:{}, y:{}, {}", x, y, buffer[0]);
                count += 1;
                //if count % width == 0 {println!("");}
            }
        }

        return ray;
    }
}
