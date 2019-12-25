fn main() {
    //IF E ELSE
    
    /*
    let control = true;

    Funciona e coloca 5 ou 6 na variável number
    let number = if control{
        5
    }
    else{
        6
    };

    println!("{}",number);

    Não Funciona pois os tipos retornados são incompatíveis
    let number = if control{
        "six"
    }
    else{
        6
    };

    println!("{}",number);
    */

    //LOOP
    /*
    let mut counter: i16 = 0;

    let result = loop{
        counter += 1;

        if (counter % 7 == 0){
            break true;
        }
        
        println!("Ainda não achamos!");
    };

    println!("{}",result);
    */

    //WHILE
    /*
    let mut number = 3;
    
    while (number > 0){
        println!("{}",number);
        number -= 1;
    }
    
    println!("sai");
    */

    //FOR
    /*
    let array = [1,2,3,4,5,6,7,8];

    for element in array.iter(){
        println!("{}",element);
    }

    for number in(0..6){
        println!("{}", array[number]);
    }
    */
}
