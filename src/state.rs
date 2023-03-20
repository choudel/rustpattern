trait Tool {
    fn mouse_down(self: Box<Self>) -> Box<dyn Tool>;
    fn mouse_up(self: Box<Self>) -> Box<dyn Tool>;
}
struct SelectionTool;
impl Tool for SelectionTool {
    fn mouse_down(self: Box<Self>) -> Box<dyn Tool> {
        println!("select tool selected");
        Box::new(SelectionTool {})
    }
    fn mouse_up(self: Box<Self>) -> Box<dyn Tool> {
        println!("draw dotted rectangle");
        Box::new(SelectionTool {})
    }
}
struct BrushTool;
impl Tool for BrushTool {
    fn mouse_down(self: Box<Self>) -> Box<dyn Tool> {
        println!("Brush tool selected");
        Box::new(BrushTool {})
    }
    fn mouse_up(self: Box<Self>) -> Box<dyn Tool> {
        println!("draw line");
        Box::new(BrushTool {})
    }
}
struct Canvas {
    state: Option<Box<dyn Tool>>,
    content: String,
}
impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            state: Some(Box::new(SelectionTool {})),
            content: String::new(),
        }
    }
    pub fn mouse_down(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.mouse_down());
        }
    }
    pub fn mouse_up(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.mouse_up());
        }
    }
}

fn main() {
    let mut canvas = Canvas::new();
    canvas.mouse_down();
    canvas.state = Some(Box::new(BrushTool {}));
    canvas.mouse_up();
}
