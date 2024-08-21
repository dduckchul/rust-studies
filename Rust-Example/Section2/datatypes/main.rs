fn main() {
	let t: (i32, bool, f64) = (32, true, 1.41);
	// 구조 분해 기법 T 에 있는 변수를 여러 변수에 할당함
	let (x, y, z) = t;
	println!("y는 {y}입니다.");

    let t: (i32, bool, f64) = (32, false, 1.41);
    let x = t.0;
    let y = t.1;
    let z = t.2;
    // 없는 변수 참조할시 컴파일 에러 발생
    //    let w = t.3;
    println!("y는 {y}입니다.");
    let x: [i32;5] = [1,2,3,4,5];
    let a = x[0]; // 1
    let b = x[4]; // 5
    
    //같은 값으로 채운 배열 array 만들기
    let fours: [i32;100] = [4; 100];
	let last = fours[99];
	println!("last word {last}")

}