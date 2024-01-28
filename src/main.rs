use std::slice;

fn main() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let value: &[i32] = unsafe {slice::from_raw_parts_mut(r, 10000)};
}


/*

fn main() {
    let mut v = vec![1, 2,3,4,5,6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [1, 2, 3]);
}
*/