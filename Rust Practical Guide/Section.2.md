## Variables
* 불변 변수 선언 let
	* let 으로 선언할경우 불변 변수, 값 넣으려고하면 컴파일러에서도 에러남 (java final 과 유사)
	* let mut 가변 변수 선언
* 상수값 const 
	* 대문자로 입력
	* 타입을 같이 선언해 줘야한다. 
	* 변경 불가
* 변수 가리기 shadowing
	* mut이 아니어도 변수를 다시 선언하면 가리는것 처럼 선언될 수 있음
	* scope 에따라 변수가 다르게 선언될수 있음
``` rust
// shadowing

let y = 4;
let y = y + 1;
println!("y의 값은 {y}입니다"); // 5출력

{
let y = y * 2;
println!("y의 값은 {y}입니다"); // 10 출력
}

println!("y의 값은 {y}입니다"); // 5출력
```

# 기본 데이터 타입
* Rust에서 모든 값은 특정 데이터 타입의 값
* 기본값 Scalar 또는 복합값 Compound
* Rust 는 정적 타입 언어
* 컴파일 시점에 모든 변수의 타입을 알아야 한다.

### 정수형 Integer 타입
|       | 부호있는  | 부호없는  |
| ----- | ----- | ----- |
| 8비트   | i8    | u8    |
| 16비트  | i16   | u16   |
| 32비트  | i32   | u32   |
| 64비트  | i64   | u64   |
| 128비트 | i128  | u128  |
| 아키텍쳐  | isize | usize |
### 정수 리터럴 표현
|        | Example     |
| ------ | ----------- |
| 10진수   | 19_384      |
| 16진수   | 0xff        |
| 8진수    | 0o77        |
| 2진수    | 0b1111_0010 |
| 바이트 u8 | b'A'        |
### 부동 소수점 (floating-point) 타입
|      | 부호있는 |
| ---- | ---- |
| 32비트 | f32  |
| 64비트 | f64  |
``` rust
let x = 3.14; // f64
let y: f32 = 3.14; // f32 
```

### 숫자 기본 연산
``` rust
let add = 3+8;
let sub = 26.5 - 2.1;
let mul = 7 * 20;
let quotient 12.0 / 3.14;
let truncated = 7 / 5; // 값이 1로나옴 (정수끼리 나눌때만)
let remainder = 46 % 5;
```

### 불린 (boolean) 타입

### char 타입
* 기본이 유니코드값
``` rust
let c = 'A';
let z: char = '가';
let unicorn = '🦄'
```
