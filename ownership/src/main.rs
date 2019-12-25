fn main() {
    
    //Testando cópia para variáveis de tamanho fixo
    let x = 5;
    let y = x;

    println!("Y está disponível: {}", y);
    println!("X está disponível: {}", x);
    
/*
    //Testando cópia com variáveis mutáveis
    let mut s1 = String::from("Hello");
    s1.push_str(", world!");
    let s2 = s1;
    println!("{}",s2);
    //println!("{}",s1);
    

    //O Ownership passou a ser S, declarado na função
    let s1 = String::from("teste");
    take_ownership(s1);
    //println!("{}",s1);
    

    let s1 = String::from("Hello");
    let s2 = takes_and_gives_ownership(s1);
    println!("{}",s2);
    */
}


fn take_ownership(s: String){
    println!("{}",s);
}

fn takes_and_gives_ownership(s: String) -> String {
    s
}