trait Button {
    fn paint(&self);
}
struct WinButton;
impl Button for WinButton {
    fn paint(&self) {
        println!("WinButton draw");
    }
}

struct OSXButton;
impl Button for OSXButton {
    fn paint(&self) {
        println!("OSXButton draw");
    }
}
trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
}
struct WinFactory;
impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton)
    }
}
struct OSXFactory;
impl GUIFactory for OSXFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(OSXButton)
    }
}
fn main() {
    let appearance = "osx";
    let factory: Box<dyn GUIFactory> = if appearance == "win" {
        Box::new(WinFactory)
    } else {
        Box::new(OSXFactory)
    };
    let button = factory.create_button();
    button.paint();
}
