use oop::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use oop::{Button, Screen};

// #![allow(unused)]
fn main() {
    // clone trait whose methods are not object safe
    // pub trait Clone {
    //     fn clone(&self) -> Self;
    // }

    // pub struct Screen {
    //     pub components: Vec<Box<dyn Clone>>,
    // }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    // screen.run();
}