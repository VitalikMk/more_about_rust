trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main(){
    println!("A baby dog is called a {}", Dog::baby_name());
}

/*use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point {x: 1, y: 0} + Point {x: 2, y: 3},
        Point {x: 3, y: 3}
    );
}*/





/*use std::slice;

fn main() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let value: &[i32] = unsafe {slice::from_raw_parts_mut(r, 10000)};
}
*/

/*

fn main() {
    let mut v = vec![1, 2,3,4,5,6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [1, 2, 3]);
}
*/