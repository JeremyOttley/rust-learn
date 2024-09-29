use std::error::Error;
use std::fmt;

fn main() -> Result<(), Box<dyn Error>> {

    let f = String::from("You will get laid!");
    let h = Horoscope(Sign::Scorpio, Fortune(f));
    let g = new_horoscope(Sign::Taurus);
    println!("{}", f);
    println!("{}", h);
    println!("{}", g);

    Ok(())
}

#[derive(Debug)]
struct Fortune(String);
//struct Fortune(&'static str);

impl fmt::Display for Fortune {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[derive(Debug)]
pub enum Sign {
  Aries,
  Taurus,
  Gemini,
  Cancer,
  Leo,
  Virgo,
  Libra,
  Scorpio,
  Sagittarius,
  Capricorn,
  Aquarius,
  Pisces,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sign::Aries => write!(f, "{}", "Aries"),
            Sign::Taurus => write!(f, "{}", "Taurus"),
            Sign::Gemini => write!(f, "{}", "Gemini"),
            Sign::Cancer => write!(f, "{}", "Cancer"),
            Sign::Leo => write!(f, "{}", "Leo"),
            Sign::Virgo => write!(f, "{}", "Virgo"),
            Sign::Libra => write!(f, "{}", "Libra"),
            Sign::Scorpio => write!(f, "{}", "Scorpio"),
            Sign::Sagittarius => write!(f, "{}", "Sagittarius"),
            Sign::Capricorn => write!(f, "{}", "Capricorn"),
            Sign::Aquarius => write!(f, "{}", "Aquarius"),
            Sign::Pisces => write!(f, "{}", "Pisces"),
        }
    }
}

#[derive(Debug)]
struct Horoscope(Sign, Fortune);


impl fmt::Display for Horoscope {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

fn new_horoscope(sign: Sign) -> Horoscope {
  let fortune = String::from("You will get lucky tonight!");
  let h = Horoscope(sign, Fortune(fortune));
  h
}
