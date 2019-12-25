fn main() {
    let s1 = String::from("Hello");

    takes_ownership(s1);

    println!("{}",s1);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}