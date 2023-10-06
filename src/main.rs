use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use vector::Vector;

fn main() {
    let window: Window = Window::new_centered("Pendulum", (800, 480)).unwrap();
    let window_handler: MyWindowHandler = MyWindowHandler {
        pendulum: Pendulum::new(400.0, 0.0, 200.0),
    };

    window.run_loop(window_handler);
}

struct MyWindowHandler {
    pendulum: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.pendulum.update();
        self.pendulum.draw(graphics);
        helper.request_redraw();
    }
}

struct Pendulum {
    origin: Vector,
    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    length: f32,
    mass: f32,
    gravity: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, length: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            length,
            mass: 5.0,
            gravity: 1.5,
        }
    }

    fn update(&mut self) {
        self.angular_acceleration =
            -1.0 * (1.0 / self.mass) * self.gravity * self.angle.sin() / self.length;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        self.position.set(
            self.length * self.angle.sin(),
            self.length * self.angle.cos(),
        );
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}
