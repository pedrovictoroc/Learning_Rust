fn main() {
    /*    
    //Testando cópia para variáveis de tamanho fixo
    let x = 5;
    let y = x;

    println!("Y está disponível: {}", y);
    println!("X está disponível: {}", x);
    

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

    //Testando conceito de empréstimo
    let s1 = String::from("hello");
    let len = get_len(&s1);
    println!("{}",len);
    

    //Testando mudança em algo passado por referencia
    //Para que possamos alterar algo passado por referencia, devemos declarar na passagem
    //a variável como do tipo mutable
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}",s1);
    */

    let mut s1 = String::from("ola mundo");

    let world = first_world(&s1);
    
    println!("{}",world);
}


fn take_ownership(s: String){
    println!("{}",s);
}

fn takes_and_gives_ownership(s: String) -> String {
    s
}

fn get_len(s: &String) -> usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str(", world!");
}

fn first_world(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}