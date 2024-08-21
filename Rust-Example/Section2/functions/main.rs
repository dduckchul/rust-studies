fn main() {
    println!("Hello, world!");
    a_function();
    print_number(3);
    add_numbers(1, 2);

    let area = circle_area(2.0);
    println!("반지름이 2.0인 원의 면적은 {area} 입니다.");
}

fn a_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_numbers(a: i32, b:i32) {
    let sum = a+b;
    println!("The sum of {} and {} is {}", a, b, sum);
}

const PI : f64 = 3.141592;

fn circle_area(radius :f64) -> f64 {
    // expression 은 세미콜론을 붙이지 않음
    PI * radius * radius
}