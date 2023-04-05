trait AbstractClass {
    fn template_method(&self) {
        self.method1();
        self.method2();
    }
    fn method1(&self) {
        println!("default impl of met1");
    }
    fn method2(&self) {
        println!("default impl of met2");
    }
}
struct ConcreteClassA;
impl AbstractClass for ConcreteClassA {
    fn method1(&self) {
        println!("custom impl of met1");
    }
}
struct ConcreteClassB;
impl AbstractClass for ConcreteClassB {
    fn method2(&self) {
        println!("custom impl of met2");
    }
}
fn main() {
    let concrete_a = ConcreteClassA;
    concrete_a.template_method();
    let concrete_b = ConcreteClassB;
    concrete_b.template_method();
}
