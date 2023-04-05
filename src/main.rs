trait Command {
    fn execute(&self);
}

struct Button {
    label: String,
    command: AddCustomerCommand,
}

impl Button {
    pub fn new(command: AddCustomerCommand) -> Self {
        Self {
            label: "Hello".to_string(),
            command,
        }
    }
    pub fn click(self) {
        self.command.execute()
    }
}
struct CustomerService;
impl CustomerService {
    fn addCustomer() {
        println!("Add customer")
    }
}
struct AddCustomerCommand;
impl Command for AddCustomerCommand {
    fn execute(&self) {
        CustomerService::addCustomer();
    }
}
fn main() {
    let service = CustomerService;
    let command = AddCustomerCommand;
    let button = Button::new(command);
    button.click();
}
