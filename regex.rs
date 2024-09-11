use regex::Regex; //crate

// doi: 10.1215/9874738291023
let re = Regex::new(r"(?P<prefix>\d{2}\.\d{4})\/(?P<isbn>\d{12})(?P<check>\d |\w)").unwrap();
