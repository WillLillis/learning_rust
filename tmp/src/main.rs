fn main() {
    let my_vec = vec![1,2,3,4];

    for i in 0..10 {
        if my_vec.contains(&i) {
            println!("Has {i}");
        } else {
            println!("No has {i}");
        }
    }

}
