trait Task {
    fn execute(&self) {
        self.record();
        self.do_execute();
    }
    fn record(&self) {
        println!("I'm recording the audit")
    }
    fn do_execute(&self);
}

struct TransferMoneyTask;
impl Task for TransferMoneyTask {
    fn do_execute(&self) {
        println!("Money tranfered")
    }
}
fn trail_audit(concrete: impl Task) {
    concrete.execute()
}
fn main() {
    trail_audit(TransferMoneyTask);
    println!();
}
