struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut x: i32 = 11;
    x = 12;
    println!("x is: {}", x); 

    let (y, z) = (1, 2);
    println!("y is: {}, z is: {}", y, z);

    println!("Hello World!");

    let x = 5;
    let y: &i32;
    y = &x;
    println!("{}", y);

    let mut point = Point{x: 110, y: 220};
    point.x = 111;
    println!("The point is at ({}, {})", point.x, point.y);
}
