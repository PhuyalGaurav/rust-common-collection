fn main() {
    let mut name: String = "Gaurav".to_string();
    println!("{}", name);

    // Since strigs are utf-8 encoded we can use nepali characters too.
    name = "गौरभ".to_string();
    println!("{}", name);

    let mut s: String = "Gulp".to_string();
    let dots: &str = " ... ";
    s.push_str(dots);
    // we can still use dots because, push_str doesnt take ownership
    println!("{}", dots);

    // Basically same but, for a single char
    s.push('.');

    let s1: String = "Hello, ".to_string();
    let s2: String = "World".to_string();

    let s3: String = s1 + &s2 + "121";
}
