struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl CircleBuilder {
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn build(&self) -> Circle {
        Circle {
            x: (self.x),
            y: (self.y),
            radius: (self.radius),
        }
    }
}

fn main() {
    let cir = Circle::new().x(1.0).y(2.0).radius(2.0).build();
    assert_eq!(cir.x, 1.0);
    assert_eq!(cir.y, 2.0);
    assert_eq!(cir.radius, 2.0);
    assert_eq!(cir.area(), 12.566370614359172);
}
