pub fn fast_floor(x: f32) -> i32 {
    let xi = x as i32;
    if x < xi as f32 {
        return xi - 1;
    }
    return xi;
}

pub fn soft_min(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * h * k / 6.0
}

pub fn map(val: f32, ai: f32, bi: f32, af: f32, bf: f32) -> f32 {
    af + (val - ai) / (bi - ai) * (bf - af)
}

/// Fast inv sqrt algorithm from the quakeIII engine
/// compute the value (1/nb)^(-0.5)
pub fn fast_isqrt(nb: f32) -> f32 {
    let x2 = nb * 0.5f32;
    let mut y = nb;
    let mut i = y.to_bits();
    i = 0x5f3759dfu32 - (i >> 1);
    y = f32::from_bits(i);
    y = y * (1.5f32 - (x2 * y * y));
    y = y * (1.5f32 - (x2 * y * y));
    return y;
}
