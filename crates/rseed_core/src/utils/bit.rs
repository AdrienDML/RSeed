pub fn bit_range_mask(start : usize, end : usize) -> u32 {
    let mut s = 1 << start;
    let offset = s;
    for _ in start..end {
        s = s << 1 | offset;
    }
    s
}