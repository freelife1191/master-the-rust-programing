# Exercise-diy-4

```rust
// 연습: 런타임 패닉을 일으키는 코드 수정
// 예상 출력:
// 큰 숫자 1: 4294967294
// 큰 숫자 2: 3000000000
// 멀. 결과: 1.2885e19
// 먼저 이 코드를 실행하고 런타임 패닉이 발생하는지 관찰합니다.
// 힌트: 'as' 키워드를 사용하거나 숫자의 데이터 유형을 변경해야 할 수도 있습니다.
fn main() {
    let large_number1:u32 = 0xffff_fffe;
    let large_number2:u32 = 3000000000;

    println!("Large Number 1: {}", large_number1);
    println!("Large Number 2: {}", large_number2);

    let result: f32 = large_number1 as f32 * large_number2 as f32;
    println!("Mul. Result: {:.4e}", result);
}
```