use std::cmp;

use crate::glyph_rays::GlyphRays;

pub fn get_ray_delta(r1: &GlyphRays, r2:&GlyphRays) -> u32 {
    let max_width = cmp::max(r1.width, r2.width) - 1;
    let max_height = cmp::max(r1.height, r2.height) - 1;
    let _debug = cmp::max(r1.r2l.len(), r2.r2l.len());

    //println!("Max width: {}. Max height: {}", max_width, max_height);
    //dispaly_vec_with_max("l2r", &r1.l2r, &r2.l2r, max_width);
    //dispaly_vec_with_max("r2l", &r1.r2l, &r2.r2l, max_width);
    //dispaly_vec_with_max("t2b", &r1.t2b, &r2.t2b, max_height);
    //dispaly_vec_with_max("b2t", &r1.b2t, &r2.b2t, max_height);
    //dispaly_vec_with_max("m2l", &r1.m2l, &r2.m2l, max_width);
    //dispaly_vec_with_max("m2r", &r1.m2r, &r2.m2r, max_width);
    //dispaly_vec_with_max("m2t", &r1.m2t, &r2.m2t, max_height);
    //dispaly_vec_with_max("m2b", &r1.m2b, &r2.m2b, max_height);

    //Horizontal vecs
    let mut horiz_delta = 0.0;
    for y in 0..=max_height {
        horiz_delta += get_vec_delta(&r1.l2r, &r2.l2r, y as usize, max_width, 0)     as f64;
        horiz_delta += get_vec_delta(&r1.r2l, &r2.r2l, y as usize, max_width, 0)     as f64;
        horiz_delta += get_vec_delta(&r1.m2r, &r2.m2r, y as usize, max_width / 2, 0) as f64;
        horiz_delta += get_vec_delta(&r1.m2l, &r2.m2l, y as usize, max_width / 2, 0) as f64;
    }
    horiz_delta = horiz_delta / max_height as f64 / max_width as f64;

    //Vertical vecs
    let mut vert_delta = 0.0;
    for x in 0..=max_width  {
        vert_delta += get_vec_delta_u8(&r1.t2b, &r2.t2b, x as usize, max_height, 2)     as f64;
        vert_delta += get_vec_delta_u8(&r1.b2t, &r2.b2t, x as usize, max_height, 2)     as f64;
        vert_delta += get_vec_delta_u8(&r1.m2b, &r2.m2b, x as usize, max_height / 2, 2) as f64;
        vert_delta += get_vec_delta_u8(&r1.m2t, &r2.m2t, x as usize, max_height / 2, 2) as f64;
    }
    vert_delta = vert_delta / max_width as f64 / max_height as f64;

    ((vert_delta + horiz_delta) * 10000.0) as u32
}

fn get_vec_delta_u8(l:&Vec<u8>, r:&Vec<u8>, index:usize, max_value:u8, stretch_limit:usize) -> u8 {
    //let max_value =
    //    if index < l.len() && index < r.len() { cmp::max(l[index], r[index]) }
    //    else if index < l.len() { l[index] }
    //    else { r[index] };
    let left =
        if index < l.len() + stretch_limit { l[cmp::min(index, l.len() - 1)] } else { max_value };
    let right =
        if index < r.len() + stretch_limit { r[cmp::min(index, r.len() - 1)] } else { max_value };
    if right > left {
        right - left
    }
    else {
        left - right
    }
}
fn get_vec_delta(l:&Vec<u16>, r:&Vec<u16>, index:usize, max_value:u16, stretch_limit:usize) -> u16 {
    //let max_value =
    //    if index < l.len() && index < r.len() { cmp::max(l[index], r[index]) }
    //    else if index < l.len() { l[index] }
    //    else { r[index] };
    let left =
        if index < l.len() + stretch_limit { l[cmp::min(index, l.len() - 1)] } else { max_value };
    let right =
        if index < r.len() + stretch_limit { r[cmp::min(index, r.len() - 1)] } else { max_value };
    if right > left {
        right - left
    }
    else {
        left - right
    }
}
