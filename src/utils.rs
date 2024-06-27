use vector2d::Vector2D;

pub fn normalize_f32(min:f32, max:f32, current:f32) -> f32 {
    if min == max {
        return min;
    }
    return (current - min) / (max - min);
}

pub fn normalize_usize(min:usize, max:usize, current:usize) -> usize {
    if min == max {
        return min;
    }
    return (current - min) % (max - min);
}

pub fn rand_range(range:usize) -> usize {
    return rand::random::<usize>() % range;
}

pub fn within(a:&Vector2D<f32>, b:&Vector2D<f32>, b_height:usize, b_width:usize) -> bool {
    let half_width = b_width as f32 * 0.5;
    let half_height = b_height as f32 * 0.5;
      return 
      ((b.x as isize - (a.x + half_width) as isize).abs() as usize) < b_width 
      && ((b.y as isize - (a.y + half_height) as isize).abs() as usize) < b_height;
}