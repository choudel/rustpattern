trait Memento<T> {
    fn restore(self) -> T;
    fn print(&self);
}
struct Originator {
    content: String,
}
struct OriginatorBackup {
    content: String,
}
impl Originator {
    pub fn save(&self) -> OriginatorBackup {
        OriginatorBackup {
            content: self.content.to_string(),
        }
    }
}
impl Memento<Originator> for OriginatorBackup {
    fn restore(self) -> Originator {
        Originator {
            content: self.content,
        }
    }
    fn print(&self) {
        println!("originator {}", self.content);
    }
}
fn main() {
    let mut history = Vec::<OriginatorBackup>::new();
    let mut originator = Originator {
        content: "hello".to_string(),
    };
    originator.content = "poss".to_string();
    history.push(originator.save());
    originator.content = "uopp".to_string();
    history.push(originator.save());
    for moment in history.iter() {
        moment.print();
    }
    let originator = history.pop().unwrap().restore();
    println!("Restored to state: {}", originator.content);
}
