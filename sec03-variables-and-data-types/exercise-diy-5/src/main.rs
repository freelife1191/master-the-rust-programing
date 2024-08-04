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

    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    println!(
        "24시간 형식으로 시간은: {:02}:{:02}:{:02}",
        hours, minutes, seconds
    );
}