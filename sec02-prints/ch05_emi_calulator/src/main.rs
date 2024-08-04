use std::io;

/*
    EMI calculator
    Fix all TODOs
*/

fn main() {
    let mut input = String::new();
    let currency_symbol = '₩'; // $, £, ¥, ₹, ₩

    //대출금액을 입력하세요.
    println!(TODOs);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let principal: f64 = input.trim().parse().expect("Input number only!");
    input.clear();

    // 연이자율을 입력합니다.
    println!(TODOs);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let annual_rate: f64 = input.trim().parse().expect("Input number only!");
    input.clear();

    // 상환 개월 수를 입력합니다.
    println!(TODOs);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let months: u32 = input.trim().parse().expect("Input number only!");
    input.clear();

    // 월간 EMI는
    let emi = calculate_emi(principal, annual_rate, months);
    println!(TODOs);
}

/*
   대출 EMI 계산 공식은 다음과 같습니다. EMI = [P x R x (1+R)^N]/[(1+R)^N-1]
   여기서 P는 원금 대출 금액입니다.
   R은 월 이자율(연 이자율을 12로 나눈 값)입니다.
   N은 월별 할부 횟수 또는 대출 기간(개월)입니다.
*/
fn calculate_emi(principal: f64, annual_rate: f64, months: u32) -> f64 {
    let monthly_rate = annual_rate / 12.0 / 100.0; // Convert percentage to a decimal and annual to monthly
    let numerator = principal * monthly_rate * (1.0 + monthly_rate).powf(months as f64);
    let denominator = (1.0 + monthly_rate).powf(months as f64) - 1.0;
    numerator / denominator
}
