use std::io::Write;

fn main() {
    print!("Hello, ");
    std::io::stdout().flush().unwrap();
    // 문자열을 표준 오류 장치로 보냄
    eprintln!("An error occurred: invalid input");
    let name = "John";
    let age = 30;
    let message = format!("My name is {} and I am {} years old", name, age);
    // 반드시 표시자가 있어야됨 변수만으로 출력할 수 없음
    println!("{}", message);
    println!("Hello, World!");
    // Console 결과 리다이렉션 cargo run > output.txt
    // 오류 메세지만 리다이렉션 cargo run 2> error.txt

    //
    // named placeholders
    //
    let message = format!("My name is {user_name} and \
                                I am {user_age} years old", user_age = age, user_name = name);
    println!("{}", message);

    //
    // print floating point values with custom decimal places
    //
    let real_value = 3.14159;
    println!("With 2 decimal places value would be {:.2}", real_value);
    println!("With 6 decimal places value would be {:.6}", real_value);
    println!("int part of the real value {} is {}", real_value, real_value as i32); // 'as' does casting

    //
    // print in hexa decimal style
    //
    let decimal_num = 6789;
    let output1 = format!("decimal number {} in hex is {:#X}", decimal_num, decimal_num);
    let output2 = format!("decimal number {} in hex is {:#x}", decimal_num, decimal_num);
    let output3 = format!("decimal number {} in hex is {:x}", decimal_num, decimal_num);
    println!("{}\n{}\n{}", output1, output2, output3);

    //
    // print in binary
    //
    println!("decimal number {num} in binary is {num:#b}", num = 6789);
    println!("decimal number {} in binary is {:#b}", 500, 500);

    //
    // 특별한 내용이 포함된 메시지를 인쇄합니다.
    // 큰따옴표("blah blah") 및 백슬래시('\')와 같은 문자
    //
    // println!("David says, "Programming is fun"");//Error
    println!("David says, \"Programming is fun\""); // 좋아요. 컴파일러가 ' " '를 이스케이프하는 데 사용되는 \에 유의하세요.

    // println!("C:\My computer\My folder");//Error
    println!("C:\\My computer\\My folder");// 좋아요. 컴파일러가 '\'를 이스케이프하는 데 사용되는 '\'

    //
    // raw string example
    //

    /* 'r' 태그가 붙은 문자열에서 이스케이프 문자 '\'가 인식되지 않기 때문에 작동합니다 */
    println!(r"C:\My computer\My folder");
    let message = r"\ \ \ \ Today is holiday \ \ \ \";
    println!("{}", message);

    /* 오류. 'r' 태그가 지정된 문자열은 큰따옴표를 포함할 수 없기 때문에 */
    // println!(r"이것은 삼중따옴표 문자열입니다. """ 이번 달은 30일입니다 """ ");

    /* 'r' 태그된 문자열 이스케이프 문자 '\'가 인식되지 않아 오류 */
    // println!(r"이것은 삼중따옴표로 묶인 문자열 \"\"\"입니다. 이번 달은 30일 \"\"\" ");

    //
    // string tagging with r#.......#
    //
    /* ####은 가독성을 위해 사용됩니다. 원하는 만큼 #을 사용할 수 있습니다. */
    let mut _message = r####"\ \ \ \ "Today is holiday" \ \ \ \"####;
    _message = r#"\ \ \ \ "Today is holiday" \ \ \ \"#;/* same as above */
    println!("{}", message);
}
