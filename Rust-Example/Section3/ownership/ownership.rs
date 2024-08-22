fn main() {
    let s = String::from("헬로");
    string_length(s);
    // main 함수를 벗어나 s는 더이상 유효하지 않음, 컴파일 에러
    // println!("s = {}", s);

    let x = 3;
    double(x);
    // 기본형은 스택 메모리에 복사가 되므로, 실행됨
    println!("x = {}", x);

    let s2 = String::from("hello");
    let s3 = string_length_return(s2);
    println!("s3 : {s3}");

    let s4 = String::from("length?");
    let (len, s4) = string_length_return_with_ownership(s4);
    println!("s4 : {s4}, len : {len}");
}

fn string_length(s:String) {
    println!("문자열 길이: {}", s.len());
}

fn string_length_return(s:String) -> String {
    println!("문자열 길이: {}", s.len());
    s
}

// 다시 소유권을 반환하는 함수
fn string_length_return_with_ownership(s:String) -> (usize, String) {
    println!("문자열 : {}", s);
    (s.len(), s)
}

fn double(x:i32) {
    println!("x 2배: {}", x * 2);
}