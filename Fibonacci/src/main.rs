fn main() {
    println!("Hello, world!");
    
    let mut squence = 123;
    let mut n = 0;
    let mut num1 = 1;
    let mut num2 = 1;
    
    print!("{}, {}, ", num1, num2);
    while n < squence {
        num1 = num1 + num2;
        num2 = num2 + num1;
        n += 1;
        print!("{}, {}, ", num1, num2);

    } 



}

