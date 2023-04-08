struct Request {
    value: i32,
}
trait Handler {
    fn handle(&self, req: &Request) -> Option<i32>;
    fn next(&mut self, next: Box<dyn Handler>);
}
struct ConcreteHandlerA {
    next: Option<Box<dyn Handler>>,
}
impl ConcreteHandlerA {
    fn new() -> Self {
        Self { next: None }
    }
}
impl Handler for ConcreteHandlerA {
    fn handle(&self, req: &Request) -> Option<i32> {
        if req.value < 10 {
            println!("this is the value needed :{}", req.value);
            Some(10)
        } else {
            match &self.next {
                Some(next) => next.handle(req),
                None => None,
            }
        }
    }
    fn next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next)
    }
}
struct ConcreteHandlerB {
    next: Option<Box<dyn Handler>>,
}
impl ConcreteHandlerB {
    fn new() -> Self {
        Self { next: None }
    }
}
impl Handler for ConcreteHandlerB {
    fn handle(&self, req: &Request) -> Option<i32> {
        if req.value < 20 {
            println!("this is the value needed :{}", req.value);
            Some(20)
        } else {
            match &self.next {
                Some(next) => next.handle(req),
                None => None,
            }
        }
    }
    fn next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next)
    }
}
struct ConcreteHandlerC {
    next: Option<Box<dyn Handler>>,
}
impl ConcreteHandlerC {
    fn new() -> Self {
        Self { next: None }
    }
}
impl Handler for ConcreteHandlerC {
    fn handle(&self, req: &Request) -> Option<i32> {
        if req.value < 30 {
            println!("this is the value needed :{}", req.value);
            Some(30)
        } else {
            match &self.next {
                Some(next) => next.handle(req),
                None => None,
            }
        }
    }
    fn next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next)
    }
}
fn main() {
    let mut request_a = ConcreteHandlerA::new();
    let mut request_b = ConcreteHandlerB::new();
    let mut request_c = ConcreteHandlerC::new();

    request_b.next(Box::new(request_c));
    request_a.next(Box::new(request_b));
    let req1 = Request { value: 15 };
    match request_a.handle(&req1) {
        Some(val) => println!("the value is :{val}"),
        None => println!("request non handled"),
    }
}
