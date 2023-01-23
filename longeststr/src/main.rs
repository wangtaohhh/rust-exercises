fn main() {
    println!("Hello, world!");

    println!("{}", longest("qwe".to_string(), "qw".to_string()));
}



fn longest(x:String, y:String) -> String {

    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }

}
