
pub trait Draw {
    fn draw(&self);
    fn name(&self) -> &String;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            println!("label={}", c.name());
            c.draw();
        }
    }
}

pub struct Button {
    pub width:  u32,
    pub height: u32,
    pub label:  String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("in {}:", self.label);
        println!("\twidth:  {}", self.width);
        println!("\theight: {}", self.height);
    }

    fn name(&self) -> &String {
        &self.label
    }
}

pub struct SelectBox {
    width:   u32,
    height:  u32,
    options: Vec<String>,
    label:   String,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("in {}:", self.label);
        for o in self.options.iter() {
            println!("\tOption: {}", o);
        }
        println!("\twidth:  {}", self.width);
        println!("\theight: {}", self.height);
    }

    fn name(&self) -> &String {
        &self.label
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 35,
                height: 64,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
                label: String::from("SelectBox"),
            }),
            Box::new(Button {
                width: 33,
                height: 66,
                label: String::from("Button"),
            }),
       ]
    };

    screen.run();
}
