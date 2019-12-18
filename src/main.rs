use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
 
struct Creature {
    tail: Vec<Vector2>,
    v: Vector2,
    dv: Vector2,
    max_speed: f32,
}

fn velocity_with_random_direction (speed: f32) -> Vector2 {
    let x = random_f32();
    let y = random_f32();
    Vector2::from((x, y)).with_magnitude(speed)
}

impl Creature {

    pub fn new() -> Self {

        let max_speed = 10.0;

        Creature {
            tail: vec![],
            v: velocity_with_random_direction(max_speed * 0.66),
            dv: Vector2::zero(),
            max_speed: max_speed
        }
    }

    fn step (&mut self, app: &App) {

        let win = app.window_rect();

        // if there are no positions in your tail yet,
        // start in the middle of the window
        let current_position = self.get_position(win);

        // the next position is current position plus current velocity
        let mut next_position = self.v + current_position;

        // wrap across the edges of the screen
        if next_position.x >= win.x.end {
            next_position.x = win.x.start;
        }
        if next_position.x <= win.x.start {
            next_position.x = win.x.end;
        }
        if next_position.y >= win.y.end {
            next_position.y = win.y.start;
        }
        if next_position.y <= win.y.start {
            next_position.y = win.y.end;
        }
        
        // They move by adding a new position to the front of the tail, and then truncating
        self.tail.insert(0, next_position);
        self.tail.truncate(100);

        // update the velocity based on acceleration
        let next_velocity = self.v + self.dv;

        // instead of simulating friction, we'll apply a speed limit to this lil dude
        let next_velocity = next_velocity.limit_magnitude(self.max_speed);

        // update the velocity
        self.v = next_velocity;

        // update acceleration
        // let's have them accelerate towards the mouse.
        let x       = next_position.x;
        let y       = next_position.y;
        let mouse_x = app.mouse.x;
        let mouse_y = app.mouse.y;

        let dv = Vector2::from((mouse_x - x, mouse_y - y));
        let dv = dv.limit_magnitude(0.7);

        self.dv = dv;

    }

    fn render (&self, draw: &nannou::app::Draw) {
        
        let length = self.tail.len();
        for i in 0..length {
            let p = self.tail[i];
            let alpha = 100.0 - i as f32;
            let size  = 120.0 - i as f32;
            let hue   = 0.3 - (i as f32 / 100.0);

            draw
            .ellipse()
            .width(size)
            .height(size)
            .xy(p)
            .hsla(hue, 0.8, 0.8, alpha / 100.0);
        }
    }

    fn get_position (&self, win: nannou::geom::rect::Rect) -> Vector2 {
        match self.tail.first() {
            Some(p) => *p,
            None => pt2(win.x(), win.y())
        }
    }
}

struct Model {
    creatures: Vec<Creature>
}

fn model(_app: &App) -> Model {
    Model {
        creatures: vec![ Creature::new() ]
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    let len = model.creatures.len();
    for i in 0..len {
        model.creatures[i].step(app)
    }

}

fn view(app: &App, model: &Model, frame: &Frame){

    frame.clear(PURPLE);

    let draw = app.draw();

    let len = model.creatures.len();
    for i in 0..len {
        model.creatures[i].render(&draw);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

}
