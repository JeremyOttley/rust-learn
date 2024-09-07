const PREFIX: &str = "A";

const SUFFIX: &str = "E";

const PPV: &str = "PPV_BV";

 

fn main() {

    let isbn: String = String::from("978-1-4780-2453-8");

    let isbn_part: &str = isbn.split("-").collect::<Vec<_>>()[3];

    let product = format!("{0}{1}{2}", PREFIX, isbn_part, SUFFIX);

    let ppv_product = format!("{0}{1}", PPV, product);

   

    println!("{}", product);

    println!("{}", ppv_product);

}
