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


        let mut grid = Grid::default();
        grid.colors[0][0] = 1;
        grid.colors[0][2] = 2;
        let canvas: Canvas<Message, Grid> = Canvas::new(grid)
            .width(Length::Units(768))
            .height(Length::Units(525));


        //縦に積み重ねる
        let explanation = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        let content = Row::new()
            .align_items(Align::Center)
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

impl<Message> canvas::Program<Message> for Grid {
    fn draw(&self, bounds: Rectangle, cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(bounds.size());

        // let width = self.square_size * colors
        let pos = Point {x: 0.0, y: 0.0};
        for i in 0..self.colors.len() {
            for j in 0..self.colors[0].len() {
            }
            println!("{:?}", self.colors[i]);
        }
        frame = self.draw(frame, pos);

        vec![frame.into_geometry()]
    }
}


impl Grid {
    const COLOR_I: Color = Color {r: 0.0, g: 255.0, b: 255.0, a: 1.0};
    const COLOR_O: Color = Color {r: 255.0, g: 255.0, b: 0.0, a: 1.0};
    const COLOR_L: Color = Color {r: 255.0, g: 165.0, b: 0.0, a: 1.0};
    const COLOR_J: Color = Color {r: 0.0, g: 0.0, b: 255.0, a: 1.0};
    const COLOR_S: Color = Color {r: 0.0, g: 255.0, b: 0.0, a: 1.0};
    const COLOR_Z: Color = Color {r: 255.0, g: 0.0, b: 0.0, a: 1.0};
    const COLOR_T: Color = Color {r: 155.0, g: 48.0, b: 0.0, a: 1.0};

    const COLOR_BACK: Color = Color {r: 181.0, g: 181.0, b: 181.0, a: 1.0};

    pub fn draw(&self, mut frame: Frame, point: Point) -> Frame {
        let mut x = 0.0 as f32;
        let mut y = 0.0 as f32;

        for column_c in &self.colors[..] { //列xの数forがまわる
            y = 0.0 as f32;
            for c in &column_c[..] { //行yの数forがまわる
                let pos_back = Point {x:x, y: y};
                let size_back = Size {width: self.square_size, height: self.square_size};
                let square_back = canvas::Path::rectangle(pos_back, size_back);
                frame.fill(&square_back, Self::COLOR_BACK);

                let pos = Point {x: x + 1.0, y: y - 1.0};
                let size = Size {width: self.square_size - 1.0, height: self.square_size - 1.0};
                let square = canvas::Path::rectangle(pos, size);
                frame.fill(&square , Self::get_color(*c));

                y += self.square_size;
            }
            x += self.square_size;
        }

        frame
    }

    fn get_color(i: usize) -> Color {
        return if i == 0 {
            Color::from_rgb8(232, 232, 232)
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
        let column_num = 20;
        let row_num = 10;
        let mut colors =  Vec::with_capacity(column_num);
        for _ in 0..row_num {
            let mut row: Vec<usize> = Vec::with_capacity(row_num);
            for _ in 0..column_num {
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