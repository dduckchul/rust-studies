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

# 복합 데이터 타입
## Tuple 타입

* 다른 데이터 타입 몇개를 확보한 쌍
``` rust 
fn main() {
	let t: (i32, bool, f64) = (32, true, 1.41);
	// 구조 분해 기법 T 에 있는 변수를 여러 변수에 할당함
	let (x, y, z) = t;
	println!("y는 {y}입니다.");
}
```

* 튜플 아이템 접근
``` rust
fn main() {
	let t: (i32, bool, f64) = (32, true, 1.41);
	let x = t.0;
	let y = t.1;
	let z = t.2;
	let w = t.3;
}
```

## 특별한 튜플 유닛 (UNIT)
* 값이 (), 함수가 반환값이 없을때 활용됨 (ex. void return);

# 배열 array 타입
``` rust
fn main() {
	let x: [i32;5] = [1,2,3,4,5]
}
```
## Tuple과 array의 큰 차이점! 
* array 똑같은 타입의 정해진 갯수의 원소를 관리한다.
* rust의 array는 가변하는 array가 없다. 고정된 크기로만 동작함
	* 가변하는 크기로 만들고 싶다면 vector가 있긴한데 나중에 이야기함

### 같은 값으로 채운 배열 만들기
```rust
fn main() {
	let fours: [i32;100] = [4; 100];
	let last = fours[99]
	println!("last word {last}")
}
```

# 함수

러스트에서는 함수명 소문자, 언더바로 구성
## 명령문 Statement 식 Expressions
* 명령문 : 뭔가 일은 하고 반환 값은 없음
* 식 : 실행 하고나면 최종 결과값이 있다.
* 함수는 여러 명령문에 이어 마지막 식으로 끝난다.
* 마지막 식은 선택적.

## 함수의 반환값 - semicolon 없이, Expression 만

## 제어구문
* if / else if 구문일반적인 제어구문과 비슷
* let에 쓰는 if 구문 -> if 구문 자체를 식으로 써버려서 반환값을 let 에 써줄수 있음
``` rust
let condition = false;
let y = if condition { 3 } else { 5 }; // y는 5
```
## 반복문 loop
``` rust
let x = loop {
	counter = counter + 1;
	if counter == 3 {
		break counter;
	}	
}
```
* loop는 일반적인 무한 반복, break 시에 상태를 반환해줄 수 있다.
### 반복문 while
* while 참인동안 반복한다 // 다른것과 비슷..
* for 문으로 반복 // 마찬가지로 비슷 
* for 문으로 숫자 범위 다루기
``` rust
let xs = [1,2,3,4,5];
let l = xs.len();
for i in (0..l) {
	println!("x={x}", xs[i]);
}
// for 문을 거꾸로
for i in (0..l).rev() {
	println!("x={x}", xs[i]);
}
```