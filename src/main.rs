use std::collections::HashMap;

fn main() {
    // This is the syntax for creating a new hashmap.
    let mut scores: HashMap<&str, i64> = HashMap::new();

    // We can add data to a hashmap using the insert method
    scores.insert("Gaurav", 500);
    scores.insert("Manish", 349);

    // Using the get method on a hashmap returns us a Option of Type i64.
    let gaurav_score: Option<&i64> = scores.get("Gaurav");
    match gaurav_score {
        Some(score) => {
            println!("Gaurav's score is {}", score);
        }
        None => {
            println!("Gaurav doesnt have a score");
        }
    }

    // Directly accessing using the unwrap_or method.
    let manish_score: i64 = scores.get("Manish").copied().unwrap_or(0);
    println!("{manish_score}");

    // Iteraating over a hashmap
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // We can update a value by overweiting it.
    scores.insert("Gaurav", 200);
    println!("{scores:?}");

    // We can check if value exist and add only if no value is found
    scores.entry("Gaurav").or_insert(50);
    println!("{:?}", scores);

    scores.entry("Himal").or_insert(50);
    println!("{:?}", scores);

    // We can change the value based on the previous value using derefrencing
    let text: &str= "Hello World this is a new experience for me. I mean a whole new fucking language this is good though ngl.";
    let mut count_words: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count_words: &mut u32 = count_words.entry(word).or_insert(0);
        *count_words += 1;
    }

    println!("{:?}", count_words);
}
