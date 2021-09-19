//! This example showcases an interactive `Canvas` for drawing Bezier curves.
use iced::{Align, Application, Button, Canvas, Checkbox, Clipboard, Color, Column, Command, Container, Element, HorizontalAlignment, Length, Point, Rectangle, Settings, Size, Text, button, canvas, executor, keyboard};

pub fn main() {
    Lienzo::run(Settings {
        antialiasing: true,
        exit_on_close_request: false,
        ..Settings::default()
    }).unwrap();
}

#[derive(Default)]
pub struct Lienzo {
    last: Vec<iced_native::Event>,
    exit: button::State,
    enabled: bool,
    should_exit: bool,
    circulo: Circulo,
    circle: canvas::Cache,
    square: Square,
    rectangle: canvas::Cache,
}


#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    KeyEventOccurred(keyboard::Event),
    Toggled(bool),
    Exit,
}

impl Application for Lienzo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Lienzo, Command<Message>) {
        (
            Lienzo {
                circulo: Circulo {radius: 50.0}, // 円の半径
                square: Square::new(50.0), // 正方形の長さ
                ..Default::default() //残りはdefault
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Simple Circle")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.last.push(event); //eventを表示するためのやつ

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }
            }
            Message::EventOccurred(event) => { //when not enabled
                if let iced_native::Event::Window(iced_native::window::Event::CloseRequested) = event {
                    self.should_exit = true;
                }
            }
            Message::KeyEventOccurred(event) => { //TODO : check its necessariness
                self.last.push(iced_native::Event::Keyboard(event));

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;
            }
            Message::Exit => {
                self.should_exit = true;
            }

        }


        /*
        let rect = self.rectangle.draw(Size {width: 1024.0, height: 700.0}, |frame| {
            println!("frame.width: {}, frame.height: {}", frame.width(), frame.height());
            // width: 1024, height: 700
            let cir = canvas::Path::circle(frame.center(), self.circulo.radius);

            frame.fill(&cir, Color::from_rgb8(0xF9, 0xD7, 0x1C));
        });
        */


        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // event listening...
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&mut self) -> Element<Message> {
        //fold で第一引数のものにfを適用する。iterの文だけやる
        let events = self.last.iter().fold( 
            Column::new().spacing(10),
            |column, event| {
                column.push(Text::new(format!("{:?}", event)).size(40))
            },
        );

        let toggle = Checkbox::new(
            self.enabled, //checkboxにより変化するflag
            "Listen to runtime events",
            Message::Toggled,
        );

        let exit = Button::new(
            &mut self.exit,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .width(Length::Units(100))
        .padding(10)
        .on_press(Message::Exit);


        /*
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill);
        */


        let content = Column::new()
            // .push(canvas)
            .align_items(Align::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

        
    }
}

#[derive(Debug)]
struct Circulo {
    radius: f32,
}

impl std::default::Default for Circulo {
    fn default() -> Self {
        Self {radius: 50.0}
    }
}

struct Square {
    size: Size,
}

impl Square {
    pub fn new(width: f32) -> Self {
        Self {size: Size {width: width, height: width}}
    }
}

impl std::default::Default for Square {
    fn default() -> Self {
        Self::new(50.0)
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