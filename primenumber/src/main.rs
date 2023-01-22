fn main() {
    println!("Hello, world!");
    for i in 2..=1000 {
        if judge(i) {
           print!("{} ", i);
        }
    }
}



fn judge(i: i32) -> bool {
    let mut i_point = i as f64;
    let mut sq = i_point.sqrt();
    let mut sq_integer = sq.floor() as i32;
    
    //let mut i_beuchushu = 1;
    let mut flag = true;

    for i_beuchushu in 2..=sq_integer {
        if i % i_beuchushu == 0 {
            //println!("{}", i);
            flag = false;
            break
        }
    }
    return flag;




}



