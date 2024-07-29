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