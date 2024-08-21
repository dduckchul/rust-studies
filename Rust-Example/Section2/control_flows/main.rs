fn main() {
    let x = 4;

    if x % 2 == 0 {
        println!("x는 짝수입니다.");
    } else {
        println!("x는 홀수입니다.");
    };

    // 평가 식으로 넣기
    let condition = false;

    let y = if condition { 3 } else { 5 };

    println!("y의 값은 {y}입니다");

    let mut counter = 0;

    let x = loop {
        println!("반복!");
        counter = counter + 1;
        if counter == 3 {
            break counter;
        }
    };

    println!("x의 값은 {x}입니다");

    let xs = [1,2,3,4,5];
    let mut idx = 0;
    while idx < xs.len() {
        println!("xs[{}] = {}", idx, xs[idx]);
        idx += 1;
    }

    for x in xs {
        println!("x = {}", x);
    }

    let l = xs.len();
    for i in 0..l {
        println!("x={}", xs[i]);
    }

    // for 문을 거꾸로
    for i in (0..l).rev() {
        println!("x={}", xs[i]);
    }
}