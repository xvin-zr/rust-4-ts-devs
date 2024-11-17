use shapes::{circle::Circle, collisions::Collidable, rect::Rect};

mod shapes;

fn main() {
    let rect = Rect::default();
    let rect2 = Rect::default();

    let circle = Circle {
        x: 1.5,
        y: 1.5,
        r: 4.0,
    };

    rect.collide(&rect2);
    circle.collide(&rect2);
    println!("{}", rect);
}
