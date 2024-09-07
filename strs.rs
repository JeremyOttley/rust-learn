// efficient for string manipulation, owned
// create and modify string data dynamically at runtime (reading/altering file content, taking in input)
let name: String = String::from("Jeremy");

// sequence of bytes, good for read-only operations, not owned
// read/analyze existing strings without making changes (parsing commandline args, searching for a substring within a string)
let name: &str = &my_string;



// // //

// create new empty string (similar to Vec<T>)
let mut s = String::new();;

//create a string with content
let data = "content" // &str
let s = data.to_string(); // turns an &str to a String

let s = String::from("content"); // String

// Update a String
let mut greet = String::from("Hello");
s.push_str(", world"); // appends a string slice to a String
s.push('!'); // appends a single character

// Concat a String
let s1 = String::from("foo ");
let s2 = String::from("bar");
// concat a reference onto a String
let s3 = s1 + &s2 // the type signature of the add function takes a string ref

// format! is preferred and safer as it always uses references no matter what you feed it
let one = String::from("Jeremy");
let two = String::from("Mark");
let three = String::from("Ottley");
let with_format = format!("{}-{}-{}", one, two, three);

// Indexing into Strings - operate on the reference
let s1 = String::from("Jeremy");
let first_char = &s1[0];

//iterate over strings (safest way is through bytes as char)
for c in "Jeremy".chars() {
	println!("{}", c);
}

for b in "杰里米".bytes() {
	println!("{}", b as char);
}

