use nannou::prelude::*;

pub struct Mover {
    pub location: Vec2,
    pub velocity: Vec2,
    pub topspeed: f32,
    pub color: Hsv,
}

impl Mover {
    pub fn new(location: Vec2, color: Hsv) -> Self {
        Self {
            location,
            velocity: Vec2::ZERO,
            topspeed: 10.0,
            color,
        }
    }

    pub fn update(&mut self, follow: &Vec2) {
        let direction = (*follow - self.location).normalize_or_zero();
        let acceleration = direction * 0.5;

        self.velocity += acceleration;
        self.velocity = self.velocity.clamp_length_max(self.topspeed);
        self.location += self.velocity;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(self.color)
            .radius(8.0)
            .xy(self.location);
    }

    pub fn check_edges(&mut self, bounds: &Rect) {
        if self.location.x < bounds.left() {
            self.location.x = bounds.right();
        } else if self.location.x > bounds.right() {
            self.location.x = bounds.left();
        }

        if self.location.y < bounds.bottom() {
            self.location.y = bounds.top();
        } else if self.location.y > bounds.top() {
            self.location.y = bounds.bottom();
        }
    }
}
