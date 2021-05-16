use crate::GlyphRays;

pub fn _print_rays(rays: &GlyphRays) -> () {
    println!();
    println!("l2r {:?}", rays.l2r);
    println!("r2l {:?}", rays.r2l);
    println!("t2b {:?}", rays.t2b);
    println!("b2t {:?}", rays.b2t);
    println!("m2l {:?}", rays.m2l);
    println!("m2r {:?}", rays.m2r);
    println!("m2t {:?}", rays.m2t);
    println!("m2b {:?}", rays.m2b);
    println!("width {:?} height {:?} top {}", rays.width, rays.height, rays.pixels_from_top);

    let height = rays.height;
    for y in 0..height {
        for x in 0..rays.width {
            if x < rays.width - 1 &&  rays.l2r[(y as usize)] == x      { print!("X"); }
            else if x > 0 && rays.r2l[y as usize] < rays.width && rays.width - 1 - rays.r2l[(y as usize)] == x { print!("X"); }
            else if y < height - 1 && rays.t2b[(x as usize)] == y { print!("X"); }
            else if y > 0 && rays.b2t[(x as usize)] < height && height - 1 - rays.b2t[(x as usize)] == y { print!("X"); }
            else if x > 0 && rays.width / 2 - rays.m2l[(y as usize)] == x { print!("X"); }
            else if x > 0 && rays.m2r[(y as usize)] + rays.width / 2 == x { print!("X"); }
            else if y > 0 && height / 2 - rays.m2t[(x as usize)] == y { print!("X"); }
            else if y < height - 1 && rays.m2b[(x as usize)] + height / 2 == y { print!("X"); }
            else { print!(" "); }
        }
        println!();
    }
}
