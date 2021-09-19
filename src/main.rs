//! This example showcases an interactive `Canvas` for drawing Bezier curves.
use iced::{Application, Canvas, Clipboard, Color, Command, Element, Length, Point, Rectangle, Sandbox, Settings, Size, canvas, executor};

pub fn main() {
    Lienzo::run(Settings {
        antialiasing: true,
        ..Settings::default()
    }).unwrap();
}

pub struct Lienzo {
    circulo: Circulo,
    circle: canvas::Cache,
    square: Square,
    rectangle: canvas::Cache,
}


#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Application for Lienzo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Lienzo {
                circulo: Circulo {radius: 50.0},
                circle: Default::default(),
                square: Square::new(50.0),
                rectangle: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Simple Circle")
    }

    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        let rect = self.rectangle.draw(Size {width: 1024.0, height: 700.0}, |frame| {
            println!("frame.width: {}, frame.height: {}", frame.width(), frame.height());
            // width: 1024, height: 700
            let cir = canvas::Path::circle(frame.center(), self.circulo.radius);

            frame.fill(&cir, Color::from_rgb8(0xF9, 0xD7, 0x1C));
        });


        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug)]
struct Circulo {
    radius: f32,
}

struct Square {
    size: Size,
}

impl Square {
    pub fn new(width: f32) -> Self {
        Self {size: Size {width: width, height: width}}
    }
}

impl<Message> canvas::Program<Message> for Lienzo {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        println!("{:?}", bounds);
        let circle = self.circle.draw(bounds.size(), |frame| {
            println!("frame.width: {}, frame.height: {}", frame.width(), frame.height());
            // width: 1024, height: 700
            let cir = canvas::Path::circle(frame.center(), self.circulo.radius);

            frame.fill(&cir, Color::from_rgb8(0xF9, 0xD7, 0x1C));


            let square_width = self.square.size.width; //squareの大きさ
            let square_height = self.square.size.height;
            let mut x = 0.0 as f32; //square の左上の座標x
            let mut y = 0.0 as f32; //square の左上の座標y
            let mut counter = 1 as usize; //色を変えるためのカウンタ
            while x + square_width <= frame.width() {
                y = 0.0 as f32;
                while y + square_height <= frame.height() {
                    let pos = Point {x: x, y: y};
                    let rect = canvas::Path::rectangle(pos, self.square.size);
                    let b = (16 * counter % 256) as u8;
                    frame.fill(&rect, Color::from_rgb8(0x00, 0x00, b));
                    counter += 1;
                    println!("counter is {}, pos = ({}, {})", counter, x, y);
                    y += square_height;
                }
                x += square_width;
            }
            let rect = canvas::Path::rectangle(frame.center(), self.square.size);

            frame.fill(&rect, Color::from_rgb8(0x00, 0xD7, 0x2C));

        });

        vec![circle]
    }

}