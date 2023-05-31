enum Flavors{
    fanta,
    pepsi,
    coke
}

struct Drink {
    flavor: Flavors,
    size: f32
}


fn get_details(drink: Drink){
    match drink.flavor {
       Flavors::fanta => println!("Flavor: fanta"),
       Flavors::coke => println!("Flavor: coke"),
       Flavors::pepsi => println!("Flavor: pepsi")
    }
    println!("{:?}", drink.size)
}


fn main(){
    let bottle = Drink{
        flavor: Flavors::pepsi,
        size: 1.5
    };
    
    get_details(bottle)
}