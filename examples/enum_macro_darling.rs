use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right(u32, u32),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

impl DirectionUp {
    fn new(speed: i32) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 10.into();
    println!("Up: {:?}, Left: {:?}", up, left);
}
