fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("32 Fahrenheit is {} Celsius", to_celsius(32.0));
    println!("0 Celsius is {} Farhenheit", to_fahrenheit(0.0));

    for n in 1..10 {
        println!("fibonacci({}) = {}", n, fibonacci(n));
    }

    // Twelve Days of Christmass
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "And a partridge in a pair tree",
        "Two turtle doves",
        "Three French hends",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for day in 0..12 {
        println!("On the {} day of Christmas", days[day]);
        println!("My true love gave to me");
        if day == 0 {
            println!("A partridge in a pair tree");
        } else {
            for gift in (0..(day + 1)).rev() {
                println!("{}", gifts[gift]);
            }
        }
        println!("");
    }
}
