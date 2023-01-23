fn main() {
    println!("Hello, world!");
    println!("{}", judge_leap(1700));



}

fn judge_leap(year: i32) -> bool {
    
    let mut flag = false;
    if year % 4 == 0 && year % 100 != 0 {
        flag = true;
    } else if year % 100 == 0 && year % 400 == 0 {
        flag = true;
    } else {
        flag = false;
    }
    return flag;

}
