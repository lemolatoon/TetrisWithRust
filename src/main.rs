//! This example showcases an interactive `Canvas` for drawing Bezier curves.
use iced::{Align, Application, Button, Canvas, Checkbox, Clipboard, Color, Column, Command, Container, Element, HorizontalAlignment, Length, Point, Rectangle, Row, Settings, Size, Text, button, canvas::{self, Frame}, executor, keyboard};

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
                column.push(Text::new(format!("{:?}", event)).size(20))
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


        let canvas: Canvas<Message, Circulo> = Canvas::new(Circulo::default())
            .width(Length::Units(512))
            .height(Length::Units(350));


        //縦に積み重ねる
        let explanation = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        let content = Row::new()
            .push(canvas)
            .push(explanation);

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

struct Grid {
    square_size: f32,
    colors: Vec<Vec<usize>>, //row * column ; 列Vec<行Vec<>>
}


impl Grid {
    const COLOR_I: Color = Color::from_rgb8(0, 255, 255);
    const COLOR_O: Color = Color::from_rgb8(255, 255, 0);
    const COLOR_L: Color = Color::from_rgb8(255, 165, 0);
    const COLOR_J: Color = Color::from_rgb8(0, 0, 255);
    const COLOR_S: Color = Color::from_rgb8(0, 255, 0);
    const COLOR_Z: Color = Color::from_rgb8(255, 0, 0);
    const COLOR_T: Color = Color::from_rgb8(155, 48, 255);

    const COLOR_BACK: Color = Color::from_rgb8(181, 181, 181);

    pub fn draw(&self, frame: Frame, point: Point) -> Frame {
        let x = 0.0 as f32;
        let y = 0.0 as f32;

        for row_c in &self.colors[..] { //行の数forがまわる
            for c in &row_c[..] { //列の数forがまわる
                let pos_back = Point {x:x, y: y};
                let size_back = Size {width: self.square_size, height: self.square_size};
                let square_back = canvas::Path::rectangle(pos_back, size_back);
                frame.fill(&square_back, Self::COLOR_BACK);

                let pos = Point {x: x - 1.0, y: y - 1.0};
                let size = Size {width: self.square_size - 1.0, height: self.square_size - 1.0};
                let square = canvas::Path::rectangle(pos, size);
                frame.fill(&square , Self::get_color(c));
            }
        }

        frame
    }

    fn get_color(i: usize) -> Color {
        return if i == 0 {
            Color::from_rgb8(176, 226, 255)
        } else if i == 1 {
            Self::COLOR_I
        } else if i == 2 {
            Self::COLOR_J
        } else if i == 3 {
            Self::COLOR_L
        } else if i == 4 {
            Self::COLOR_O
        } else if i == 5 {
            Self::COLOR_S
        } else if i == 6 {
            Self::COLOR_T
        } else if i == 7 {
            Self::COLOR_Z
        } else {
            panic!("illegal color number: {}", i);
        };
    }
}

impl std::default::Default for Grid {
    fn default() -> Self {
        let colors =  Vec::with_capacity(20);
        for color in colors {
            let row:Vec<usize> = Vec::with_capacity(10);
            for i in 0..10 {
                row.push(0);
            }
            colors.push(row);
        } // init (0-fill)

        Self {
            square_size: 20.0,
            colors: colors,
        }
    }
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

impl<Message> canvas::Program<Message> for Circulo {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        println!("{:?}", bounds);

        // prepare new frame
        let mut frame = Frame::new(bounds.size());

        println!("frame.width: {}, frame.height: {}", frame.width(), frame.height());
        // width: 1024, height: 700
        let cir = canvas::Path::circle(frame.center(), self.radius);

        frame.fill(&cir, Color::from_rgb8(0xF9, 0xD7, 0x1C));


        let square_width = 50.0; //squareの大きさ
        let square_height = 50.0;
        let mut x = 0.0 as f32; //square の左上の座標x
        let mut y = 0.0 as f32; //square の左上の座標y
        let mut counter = 1 as usize; //色を変えるためのカウンタ
        while x + square_width <= frame.width() {
            y = 0.0 as f32;
            while y + square_height <= frame.height() {
                let pos = Point {x: x, y: y};
                let size = Size {width: self.radius, height: self.radius}; // 半径と同じ長さの正方形
                let rect = canvas::Path::rectangle(pos, size);

                let b = (16 * counter % 256) as u8;
                frame.fill(&rect, Color::from_rgb8(0x00, 0x00, b));
                counter += 1;
                println!("counter is {}, pos = ({}, {})", counter, x, y);
                y += square_height;
            }
            x += square_width;
        }
        let rect = canvas::Path::rectangle(frame.center(), Size {width: 50.0, height: 50.0});

        frame.fill(&rect, Color::from_rgb8(0x00, 0xD7, 0x2C));


        vec![frame.into_geometry()]
        }

}