use crate::entities::bounds::Bounds;

pub struct ActorConfig {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub box_size: f32,
}

pub struct Actor {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub box_size: f32,
}

impl Actor {
    pub fn new(id: u32, x: f32, y: f32, config: ActorConfig) -> Self {
        Self {
            id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            max_speed: config.max_speed,
            acceleration: config.acceleration,
            deceleration: config.deceleration,
            box_size: config.box_size,
        }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(self.id, self.x, self.y, self.box_size, self.box_size)
    }

    pub fn apply_input(&mut self, dx: f32, dy: f32, dt: f32) {
        let accel = self.acceleration * dt;
        let drag = (1.0 - self.deceleration * dt).max(0.0);

        if dx < 0.0 {
            self.vx -= accel;
        } else if dx > 0.0 {
            self.vx += accel;
        } else {
            self.vx *= drag;
        }
        if dy < 0.0 {
            self.vy -= accel;
        } else if dy > 0.0 {
            self.vy += accel;
        } else {
            self.vy *= drag;
        }
        self.vx = self.vx.clamp(-self.max_speed, self.max_speed);
        self.vy = self.vy.clamp(-self.max_speed, self.max_speed);
    }
}
