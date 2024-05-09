pub fn within(a:u16, b:u16, size:u16) -> bool {
    let diff = a - b;
    return diff > size;
}

pub fn normalize(min:f32, max:f32, current:f32) -> f32 {
    return (current - min) / (max - min);
}