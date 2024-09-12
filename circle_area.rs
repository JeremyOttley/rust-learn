type Area = f64;
type Radius = f64;

fn circle_area(radius: Radius) -> Area {
    std::f64::consts::PI * (radius * radius)
}

