#[derive(Debug)]
struct Coordinate(f64, f64);

impl Coordinate{

    fn new(longitude: f64, latitude: f64) -> Self {
        Coordinate(longitude, latitude)
    }

	fn get_longitude(&self) -> f64 {
		self.0
	}

	fn get_latitude(&self) -> f64 {
		self.1
	}

	fn geolocate(&self) -> String {
		if self.0 + self.1 >= 0.0 {
			"Cannot locate...".to_string()
		} else {
			"Baltimore!".to_string()
		}
	}
}

fn main() {
	let city = Coordinate::new(39.3, -76.6);
	println!("You can find Baltimore on the map at {} by {}", city.get_longitude(), city.get_latitude());
	println!("{}", city.geolocate());

}
