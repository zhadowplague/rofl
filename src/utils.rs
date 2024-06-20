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
    return (current - min) / (max - min);
}