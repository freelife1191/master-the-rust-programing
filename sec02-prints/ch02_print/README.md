## Other flavours of println!

- `print!`: `println!`과 비슷하지만 출력 끝에 새 줄을 추가하지 않습니다. 같은 줄에 여러 항목을 인쇄하려는 경우 유용할 수 있습니다
- `eprintln!`: `println!`과 유사하지만 표준 출력 스트림 대신 표준 오류 스트림에 인쇄합니다. 이는 오류 메시지를 인쇄하는 데 유용할 수 있습니다
- `format!`: 주어진 형식으로 새로운 문자열을 생성합니다. 이는 여러 값을 포함하는 문자열을 만들거나 고급 형식 지정 옵션을 사용하려는 경우에 유용할 수 있습니다
- `write!`: `format!`과 비슷하지만 새 문자열을 반환하는 대신 버퍼에 씁니다

- 매크로 이름 끝에 `!`는 함수가 아닌 매크로임을 나타내며 매크로는 Rust 표준 라이브러리의 일부입니다
- 위의 모든 기능은 매크로이며 자리 표시자를 사용하여 출력 형식을 지정하는 데 사용할 수 있습니다. 이러한 자리 표시자는 `{}`로 표시됩니다


## Example

다음은 하나의 기능에서 다양한 인쇄 변형을 사용하는 방법에 대한 예입니다

```rust
fn main() {
    print!("Hello, ");
    eprintln!("An error occurred: invalid input");
    let name = "John";
    let age = 30;
    let message = format!("My name is {} and I am {} years old", name, age);
    println!("{}", message);
    println!("Hello, World!");
}
```

Output:

```shell
Hello, An error occurred: invalid input
My name is John and I am 30 years old
Hello, World!
```


## eprintln!

* 개행 문자를 사용하여 표준 오류 스트림으로 인쇄합니다
* 출력이 `std::stdout` 대신 `std::stderr`로 가는 점을 제외하면 `println!` 매크로와 동일합니다
* 오류 및 진행 메시지에만 `eprintln!`을 사용하세요. 프로그램의 기본 출력 대신 `println!`을 사용하세요
* 기본적으로 표준 출력 및 표준 오류 스트림이 모두 터미널에 표시됩니다
* 터미널은 프로그램이 출력을 쓸 수 있는 두 개의 별도 스트림을 제공합니다. 하나는 일반 출력용(`stdout`)이고 다른 하나는 오류 메시지용(`stderr`)입니다
* 표준 오류 또는 표준 출력 스트림은 `>`(stdout 메시지 리디렉션) 또는 `2>`(stderr 메시지 리디렉션)와 같은 리디렉션 연산자를 사용하여 별도의 파일로 리디렉션될 수 있습니다

https://doc.rust-lang.org/stable/std/macro.eprintln.html