


하루 중 초 단위의 시간을 정수로 취하고 이를 24시간 형식(HH:MM:SS)으로 변환하는 Rust 프로그램을 작성하세요

예상 출력:
```
Enter the time of the day in seconds(0 to 86,399):

81000

Time in 24hr format is: 22:30:00
```


Hints:

1. HH:MM:SS의 3개 필드 값을 계산해야 합니다. HH는 시간, MM은 분, SS는 초를 나타냅니다
2. 시간(HH)을 계산하려면 아래 공식을 사용하십시오
   - `Hours = total_seconds/3600;`
   - 참고: `total_seconds`는 사용자가 입력한 값입니다
3. mod(%) 연산자를 사용하여 남은 시간(초)을 알아보세요.
   - `remaining_seconds = total_seconds % 3600`
4. 분(MM)을 계산하려면 아래 공식을 사용하십시오.
   - `Minutes = remaining_seconds / 60`
5. Seconds(SS)이기도 한 mod(%) 연산자를 사용하여 남은 초를 알아냅니다
   - `Seconds= remaining_seconds % 60`
6. HH:MM:SS 형식으로 인쇄하세요

**Use the partial code given**

```rust
/*
    Convert seconds to HH:MM:SS format
*/
use std::io;

fn main() {
    let mut input = String::new();

    println!("하루 중 시간을 초 단위로 입력하세요.(0 to 86,399):");
    io::stdin()
        .read_line(&mut input)
        .expect("줄을 읽지 못했습니다.");
    let total_seconds: u32 = input
        .trim()
        .parse()
        .expect("부호 없이 숫자만 입력하세요!");

    if total_seconds > 86399 {
        panic!("입력값은 0에서 86,399 사이여야 합니다. ");
    }

    // TODO
}
```