const PI : f32 = 3.141592;
fn main() {
    // 변수 선언, 변경 안됨
    // let x = 3;
    let mut x = 3;
    println!("x의 값은 {x}입니다");
    x = 7;
    println!("x의 값은 {x}입니다");

    // shadowing
    let y = 4;
    let y = y + 1;
    println!("y의 값은 {y}입니다");
    {
        let y = y * 2;
        println!("y의 값은 {y}입니다");
    }

    println!("y의 값은 {y}입니다");

    println!("원주율의 값은 {PI}입니다");
}

