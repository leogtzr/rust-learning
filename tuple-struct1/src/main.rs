struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32, 
    height: u32,
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    println!(".0 = {}", black.0);
    println!(".1 = {}", black.1);
    println!(".2 = {}", black.2);

    let Point(x, y, z) = point;
    println!("x = {x}");
    println!("y = {}", y);
    println!("z = {}", z);

    struct AlwaysEqual;
    let subject = AlwaysEqual;

    let rect: (u32, u32) = (30, 50);
    let rect_area = area(rect);

    println!("Area is: {}", rect_area);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let a_rect: u32 = area_rectangle(&rect1);
    println!("Result is: {}", a_rect);

    println!("{}", rect1.width);
}
