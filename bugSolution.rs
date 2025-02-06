fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    if vec.len() > 2 {
        let value = vec[2];
        println!("The value is: {}", value);
    } else {
        println!("Vector does not have an element at index 2");
    }
    //Alternative using get
    match vec.get(2) {
        Some(value) => println!("The value is: {}", value),
        None => println!("Vector does not have an element at index 2"),
    }
} 