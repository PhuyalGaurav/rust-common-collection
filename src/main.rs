fn main() {
    let mut names: Vec<&str> = vec!["gaurav", "manish", "shuva"];

    // Updating a vector.
    names.push("arnav");

    let second: &str = names[1];

    println!("The Second element is {}", second);

    let fourth: Option<&&str> = names.get(2);

    println!("{:?}", fourth);
    match fourth {
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => println!("No fourth element"),
    }

    let non_existant_vector: &&str = &names[1];

    println!("{}", non_existant_vector);

    for name in names {
        println!("{}", name);
    }

    enum Numbers {
        Int(i64),
        Float(f64),
        Name(String),
    }

    let nums: Vec<Numbers> = vec![
        Numbers::Int(30),
        Numbers::Float(3.14),
        Numbers::Name(String::from("Thirty Two")),
    ];

    for num in nums {
        match num {
            Numbers::Int(z) => {
                println!("{}", z);
            }
            Numbers::Float(x) => {
                println!("{}", x);
            }
            Numbers::Name(y) => {
                println!("{}", y);
            }
        }
    }
}
