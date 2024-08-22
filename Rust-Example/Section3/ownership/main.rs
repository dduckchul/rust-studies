fn main() {
    let mut s: String = String::from("헬로");
    s.push_str(", 월드!");
    println!("s = {}", s);

    {
        // 기본값들은 stack에 저장됨
        let x = 3;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }

    {
        let s1 = String::from("hello"); // heap 메모리에 저장됨
        println!("s1 = {}", s1);

        let s2 = s1;
        println!("s2 = {}", s2);
        // 컴파일 안됨, s1은 이미 소유권이 s2로 이동되었기 때문에
        // let s2 = s1.clone(); // 복사해서 쓸 경우
        // println!("s1 = {}", s1);
    }
}
