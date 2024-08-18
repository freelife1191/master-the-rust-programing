use std::io::Write;

fn main() {
    print!("Hello, ");
    std::io::stdout().flush().unwrap();
    // 문자열을 표준 오류 장치로 보냄
    // https://doc.rust-lang.org/stable/std/io/fn.stderr.html
    eprintln!("An error occurred: invalid input");
    eprintln!("An error occurred: invalid entry");
    let name = "John";
    let age = 30;
    let message = format!("My name is {} and I am {} years old", name, age);
    // 반드시 표시자가 있어야됨 변수만으로 출력할 수 없음
    println!("{}", message);
    println!("Hello, World!");
    // Console 결과 리다이렉션 cargo run > log.txt
    // 오류 메세지만 리다이렉션 cargo run 2> error.txt
}
