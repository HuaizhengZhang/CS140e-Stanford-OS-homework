fn main() {
    println!("Hello, world!");

    if_exp();

    test_if_con();

    test_loop();

    test_while();

    test_for();

    test_iter();

    test_rev();
}


fn if_exp() {

    let num = 7;

    if num < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
}

fn test_if_con() {

    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };


    println!("The value of test_if_con is {}", number);

}

fn test_iter() {

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the iter value is {}", element);
    }
}


fn test_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("test loop result is {}", result);
}

fn test_while() {
    let mut number = 3;

    while number != 0 {

        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}


fn test_for() {

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {

        println!("the value of test_for is {}", a[index]);

        index = index + 1
    }
}


fn test_rev() {

    for number in (1..4).rev() {
        println!("rev is {}", number);
    }

    println!("LIFTOFF!!!");
}