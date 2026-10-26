fn main() {
    let test_variant = 0;

    let a = if test_variant == 1 {
        test_variant + 1
    } else {
        1
    };

    let number = 0;
    if number < 0 {
        println!("Число {number} отрицательное");
    } else if number > 0 && number < 10 {
        println!("Число {number} положительное и меньше 10");
    } else if number == 10 {
        println!("Число равно десяти");
    } else if number > 10 {
        println!("Число {number} > 10");
    } else {
        println!("Число не подошло к условиям");
    }
    
    println!("У вас вариант {a}")

}
