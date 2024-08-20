# Master The Rust Programming Language : Beginner To Advanced

- https://www.udemy.com/course/master-the-rust-programming-language/?couponCode=JUST4U02223
- https://github.com/niekiran/Rust


## Rust 공식 추천 도구

- Rust 형식(`rustfmt`): 커뮤니티에 따라 Rust 코드의 형식을 자동으로 지정
    - 프로젝트 전체의 코드가 일관된 스타일을 갖도록 보장하며 이는 특히 협업에 도움이 됨
- Rust 문법(`clippy`): 코드를 더 나은 방향으로 개선할 수 있는 린트 도구
    - Cargo Build는 내장된 정적 분석 규칙이나 린트 검사를 사용한다
    - Rust 코드를 실행하지 않고 분석하여 Rust 컴파일러 자체에서 발생하는 광범위한 문제를 찾아냄
- `cargo fix`: 코드를 자동으로 수정하는 도구
    - 코드를 수정하는 데 도움이 되는 린트 규칙을 적용하여 코드를 수정할 수 있음


#### VS CODE 에 Clippy 설치

settins.json 파일에 아래 내용 추가

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.enable": true
}
```

## Rust Programming Language

1. Rust는 시스템 소프트웨어 작성을 위해 설계된 시스템 프로그래밍 언어입니다. 빠르고 효율적이며 신뢰할 수 있도록 개발되었습니다
2. Rust는 정적으로 유형이 지정됩니다. 이는 변수 유형이 런타임이 아닌 컴파일 타임에 알려짐을 의미합니다. 이를 통해 보다 효율적이고 예측 가능한 성능을 얻을 수 있습니다
3. Rust는 컴파일된 언어입니다. 즉, 소스 코드를 실행하려면 먼저 기계어 코드로 번역해야 합니다
4. Rust는 안전성, 동시성, 메모리 관리를 강조합니다. 소유권 및 차용 시스템을 통해 메모리 안전을 제공하므로 널 포인터 역참조 및 버퍼 오버플로와 같은 일반적인 프로그래밍 오류를 방지하는 데 도움이 됩니다


### Support for OOP

Rust는 구조(구조체)와 특성을 사용하여 객체 지향 프로그래밍(OOP)을 지원합니다    
OOP에 대한 Rust의 접근 방식은 Java나 C++와 같은 기존 OOP 언어만큼 완전히 개발되지는 않았지만 일반적인 OOP 디자인 패턴을 구현하는 데 충분한 기능을 제공합니다    
Rust의 소유권 및 차용 모델은 안전하고 예측 가능한 객체 수명을 보장하는 데 도움이 되므로 OOP 프로그래밍에 적합한 선택입니다


## Rust 는 다른 프로그래밍 언어와 어떻게 다른가요?

- `Concurrency`: Rust에는 동시성 지원 기능이 내장되어 있어 여러 스레드에서 코드를 안전하게 실행할 수 있습니다. 이는 다른 언어에서 사용되는 공유 상태 동시성과 달리 가벼운 "tasks"와 메시지 전달 동시성을 통해 달성됩니다
- `Package manager`: Rust에는 라이브러리와 패키지를 쉽게 찾고, 사용하고, 공유할 수 있게 해주는 "Cargo"라는 패키지 관리자가 내장되어 있습니다. 이를 통해 효율적인 개발 및 코드 재사용이 가능하며 강력한 커뮤니티 중심 생태계를 촉진합니다
- `Strong typing`: 즉, 변수에는 특정 유형이 있으며 해당 유형을 존중해야 합니다. 이는 컴파일러가 모든 유형 불일치에 플래그를 지정하므로 오류를 조기에 포착하는 데 도움이 되며 코드를 더욱 예측 가능하게 만듭니다
- `No runtime`: Rust에는 런타임도 없고 가비지 수집기도 없습니다. 이를 통해 프로그램에 대한 더 많은 제어가 가능하고 런타임이 필요하지 않으므로 성능이 저하되고 가비지 수집기의 복잡성이 추가됩니다
- `Error handling`: Rust는 오류 처리에 대한 독특한 접근 방식을 가지고 있어 오류를 안전하고 명시적으로 처리할 수 있습니다. Rust 에서 오류는 특정 유형으로 표시되며, 이는 프로그래머가 Result 유형 및 `?` 연산자
- `Cross-platform compatibility`: Rust는 Windows, Linux, macOS 및 심지어 임베디드 시스템을 포함한 다양한 플랫폼에서 실행되는 코드를 작성하는 데 사용될 수 있습니다.  
  따라서 웹 개발, Linux 커널 개발, 게임 개발 및 IoT를 포함한 다양한 유형의 프로젝트 및 시스템에 적합한 다용도 언어입니다


### 메모리 안전성

메모리 안전은 프로그램이 정의되지 않은 동작을 일으키지 않거나 메모리에 대한 잘못된 액세스로 인해 충돌이 발생하지 않도록 보장하는 것을 의미합니다  
Rust는 엄격한 소유권 모델, 자동 참조 계산, 한 번에 프로그램의 한 부분에서만 메모리 조각에 액세스할 수 있도록 보장하는 빌림 검사기, `null`과 같은 일반적인 프로그래밍 오류 방지, 데이터 경합 및 매달린
포인터 참조 등의 기능을 결합하여 이를 달성합니다

#### Rust는 자동으로 힙 메모리 할당을 해제합니다

- Rust에서는 일반적으로 C++에서처럼 `new` 및 `free`와 같은 함수를 사용하여 힙 메모리를 수동으로 관리하지 않습니다
- Rust는 소유권 시스템을 통해 메모리 관리를 자동화합니다. 각 데이터 조각에는 명확하고 컴파일 시간에 정의된 소유자가 있으며, 소유자가 범위를 벗어나면 메모리 할당이 자동으로 해제됩니다

### Ownership model

- Rust 에서 소유권은 Rust 컴파일러가 메모리에 있는 값의 수명을 관리하는 방식을 나타냅니다. 소유권 규칙에 따르면 값에는 소유자가 한 명만 있을 수 있으며 소유자가 범위를 벗어나면 값이 자동으로 삭제됩니다
- 이러한 규칙을 시행함으로써 Rust는 데이터 경합 및 기타 일반적인 프로그래밍 오류를 방지하여 시스템 프로그래밍을 위한 안전하고 효율적인 언어로 만듭니다

### 동등한 C++ 코드

![img.png](attachments/img1.png)

- Rust 에서 `String`의 소유권을 한 변수에서 다른 변수로 이전하는 것은 명시적인 `std::move`가 필요 없이 기본적으로 자동으로 발생합니다
- Rust 에서는 소유권 이전 후 원래 `String`이 더 이상 유효하지 않으며, 이를 사용하려고 하면 컴파일 타임 오류가 발생합니다

### Rust는 매달린 포인터를 피합니다

![img.png](attachments/img2.png)

- 코드는 경고 `Wreturn-local-addr`과 함께 컴파일됩니다
- 'p'에는 유효하지 않은 주소가 있습니다
- 'p'를 역참조하면 원하지 않는 동작이나 충돌이 발생합니다

### Type inference

Rust 에는 `Type inference` 기능이 있습니다    
즉, 컴파일러는 명시적으로 지정할 필요 없이 변수나 표현식의 유형을 자동으로 추론할 수 있습니다    
이렇게 하면 코드를 더 읽기 쉽고 덜 장황하게 만들 수 있습니다

```
let x = 42; // The type of `x` is inferred to be `i32`
int x = 42; // The type of `x` is explicitly set to `int`
```

### Error handling

- Rust는 C++와 같은 예외 대신 반환 값 기반 오류 처리를 사용합니다
- Rust에서 오류는 일반적으로 `Result` 및 `Option` 유형으로 표시되어 함수에서 성공, 오류 또는 값 없음 조건을 반환합니다
- Rust 프로그래밍 언어는 정의되지 않은 동작을 방지하기 위해 명시적인 오류 처리 및 컴파일 타임 검사를 사용하는 반면 C++는 예외에 의존합니다


## Rust 자료형에 대응하는 Java 자료형

| **Rust 자료형**    | **Java 자료형**                       |
|-----------------|------------------------------------|
| `i8`            | `byte`                             |
| `i16`           | `short`                            |
| `i32`           | `int`                              |
| `i64`           | `long`                             |
| `i128`          | `BigInteger`                       |
| `isize`         | `int` (32-bit) 또는 `long` (64-bit)  |
| `u8`            | `short` (부호 없는 정수형은 Java에 없음)      |
| `u16`           | `int` (부호 없는 정수형은 Java에 없음)        |
| `u32`           | `long` (부호 없는 정수형은 Java에 없음)       |
| `u64`           | `BigInteger` (부호 없는 정수형은 Java에 없음) |
| `usize`         | `int` (32-bit) 또는 `long` (64-bit)  |
| `f32`           | `float`                            |
| `f64`           | `double`                           |
| `bool`          | `boolean`                          |
| `char`          | `char`                             |
| `&str`          | `String`                           |
| `String`        | `String`                           |
| `[T; N]`        | `T[]`                              |
| `(T1, T2, ...)` | `class` (사용자 정의 클래스)               |


## Rust 문서 모음

- 러스트 북 커뮤니티 번역본: https://doc.rust-kr.org/
- 러스트 북 원본 영문: https://doc.rust-lang.org/stable/book/
- 러스트 북 Summary: https://codeahoy.com/learn/tutorials/rust-book-summary/
- 러스트 북 Example Solutions
    - https://github.com/kmoschcau/rust-book-exercises
    - https://github.com/olehmisar/The-Rust-Programming-Language-Book-Solutions
    - https://github.com/laercioxlaercio/rust
    - https://github.com/kevinalh/rust-book
    - https://github.com/klemola/rust-book
    - https://github.com/jasonkuhrt-archive/rust-book-exercises
    - https://github.com/Lukman-01/rust-learn-by-practice
    - https://github.com/rust-unofficial/rust-practise-questions/tree/master/src
- 러스트 북 Quiz 버전: https://rust-book.cs.brown.edu/
- 표준 라이브러리 문서: https://www.rust-lang.org/learn
- Rust API Document: https://doc.rust-lang.org/std/
- Rust Project: https://www.rust-lang.org/
- Rust Document: https://doc.rust-lang.org/beta/
    - Rust Reference: https://doc.rust-lang.org/beta/reference/index.html
    - The Edition Guide: https://doc.rust-lang.org/beta/edition-guide/editions/index.html
    - The Release Notes: https://doc.rust-lang.org/beta/releases.html
    - The rustc Book: https://doc.rust-lang.org/rustc/
    - The Cargo Book: https://doc.rust-lang.org/beta/cargo/index.html
    - The Rustdoc Book: https://doc.rust-lang.org/beta/rustdoc/index.html
    - The Clippy Book: https://doc.rust-lang.org/beta/clippy/index.html
    - rustc error codes: https://doc.rust-lang.org/beta/error_codes/index.html
    - The Style Guide: https://doc.rust-lang.org/beta/style-guide/index.html
    - The Rustonomicon: https://doc.rust-lang.org/beta/nomicon/index.html
    - The Unstable Book: https://doc.rust-lang.org/beta/unstable-book/index.html
    - Rust Compiler Developer Guide: https://rustc-dev-guide.rust-lang.org/
    - Rust Embedded: https://github.com/rust-embedded
    - The Embedded Rust Book: https://doc.rust-lang.org/beta/embedded-book/index.html
- Rust Docs 검색: https://docs.rs/
- Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021
- Rust 예제: https://doc.rust-lang.org/rust-by-example/
- Rust 예제 & 해답: https://github.com/rust-lang/rustlings
    - https://github.imc.re/topics/rustlings-solution?o=asc&s=stars
- Rust By Practice: https://practice.course.rs/why-exercise.html
- 100 Exercises To Learn Rust: https://rust-exercises.com/100-exercises/