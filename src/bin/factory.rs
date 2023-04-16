trait Shape {
    fn draw(&self);
}
struct Circle;
impl Shape for Circle {
    fn draw(&self) {
        println!("draw circle");
    }
}
struct Square;
impl Shape for Square {
    fn draw(&self) {
        println!("draw square");
    }
}

enum ShapeType {
    Circle,
    Square,
}
struct ShapeFactory;
impl ShapeFactory {
    fn get_shape(&self, shape_type: ShapeType) -> Box<dyn Shape> {
        match shape_type {
            ShapeType::Circle => Box::new(Circle),
            ShapeType::Square => Box::new(Square),
        }
    }
}
fn main() {
    let shape_factory = ShapeFactory;
    let shape1 = shape_factory.get_shape(ShapeType::Circle);
    shape1.draw();
    let shape2 = shape_factory.get_shape(ShapeType::Square);
    shape2.draw();
}
