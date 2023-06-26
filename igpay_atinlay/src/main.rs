use std::io;

fn main() {
    println!("Enter a word to be converted.");

    let mut my_str = String::new();

    io::stdin().read_line(&mut my_str).expect("Failed to read line.");
    my_str.pop(); // remove the newline character at the end
    
    to_pig_latin(&mut my_str);

    println!("{}", my_str);
}

fn to_pig_latin(str_in: &mut String) {
    match str_in.chars().nth(0).expect("Uh oh") {
    'a'|'e'|'i'|'o'|'u'|'y'|'A'|'E'|'I'|'O'|'U'|'Y' => {
            str_in.push_str("-hay");
        },
    _ => {
            str_in.push('-');
            let first = str_in.remove(0).to_ascii_lowercase();
            str_in.push(first);
            str_in.push_str("ay");
        },
    }
}
