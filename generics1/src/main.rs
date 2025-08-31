struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Using a concrete type.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // Point the number to the first element.
    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let max = largest(&number_list);

    println!("The largest number is: {}", max);

    let integer: Point<i32> = Point { x: 1, y: 2};
    println!("integer.x = {}", integer.x);
    println!("integer.y= {}", integer.y);

    let x: Point<f32> = Point {
        x: 1.2, y: 5.3,
    };

    let y = x.distance_from_origin();
    println!("Value: {}", y);
    
}
