let v = vec![1, 2, 3];
match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => eprintln!("Sorry, this vector is too short.")
}
