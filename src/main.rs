pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    pub fn add_component(&mut self, component: Box<(dyn Draw)>) {
        self.components.push(component)
    }
}

pub struct Button {}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button");
    }
}

pub struct SelectBox {}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box");
    }
}
fn main() {
    let screen = {
        let mut screen = Screen { components: vec![] };
        screen.add_component(Box::new(Button {}));
        screen.add_component(Box::new(SelectBox {}));
        screen
    };
    screen.run();
}
