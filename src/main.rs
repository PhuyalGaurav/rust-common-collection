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

    // s1 cant be used becuase it is moved but s2 can since only a refrence is passe in the + operator
    println!("{s2}");

    // if we dont want to take ownership of anything we can use format macro
    let format_s: String = format!("{s2}-{s3}");
    println!("{format_s}");

    let hello: &str = "नमस्ते";
    // rust doesnt support indexing so we must slice it.
    let aadi: String = hello[9..12].to_string();
    println!("{aadi}");

    //let aadi2: String = hello[0..1].to_string();
    // This wont compile because indivisual bytes is not stored in strings
    // error : byte index 1 is not a char boundary; it is inside 'न' (bytes 0..3) of `नमस्ते`|

    // We can use the bytes indivisually by iterating over it :
    for b in hello.bytes() {
        println!("{b}");
    }
}
