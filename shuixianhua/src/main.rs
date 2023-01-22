fn main() {
    println!("Hello, world!");
    for i in 1..10000 {
        let mut gewei:i32 = i % 10;
        let mut shiwei:i32 = i / 10 % 10;
        let mut baiwei:i32 = i / 100 % 10;
        let mut qianwei:i32 = i / 1000 % 10;
       
        if gewei.pow(3) + shiwei.pow(3) + baiwei.pow(3) + qianwei.pow(3) == i{
            println!("{}", i);
        }
    }
    println!("-------------------------------------------");

    sxh(1000000);
}


fn sxh(number: i32){

    for num in 1..number{

    let num_str = num.to_string();
    let len_num_str = num_str.len();
    let mut number_in_num = 0;
    let mut sum = 0; 
    for slice_num in 0..len_num_str {
        let mut string_value = &num_str[slice_num..slice_num+1];
        let mut string_int = string_value.parse::<i32>().unwrap();
        sum += string_int.pow(3);
        if sum == num {
        println!("{}", sum);
    }

   
    }


    }
}




  




















