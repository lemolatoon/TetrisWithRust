mod tetris_core;

use iced::time;
use rand::Rng;
use tetris_core::mino::Mino;
use tetris_core::mino::Minos;
use tetris_core::mino;

use chrono;

use iced::{Align, Application, Button, Canvas, Checkbox, Clipboard, Color, Column, Command, Container, Element, HorizontalAlignment, Length, Point, Rectangle, Row, Settings, Size, Text, button, canvas::{self, Frame}, executor, keyboard};
use iced_native;

pub fn main() {
    Lienzo::run(Settings {
        antialiasing: true,
        exit_on_close_request: false,
        ..Settings::default()
    }).unwrap();
}

pub struct Lienzo {
    last: Vec<iced_native::Event>,
    exit: button::State,
    enabled: bool,
    should_exit: bool,
    grid: Grid,

    rng: rand::rngs::ThreadRng,
}


#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    Tick(chrono::DateTime<chrono::Local>),
    // ToDo(Vec<MessageZipper>),
    Toggled(bool),
    Exit,
}

#[derive(Debug, Clone)]
enum MessageZipper {
    EventOccurred(iced_native::Event),
    Tick(chrono::DateTime<chrono::Local>),
}

impl Lienzo {
    const DOPT_DELTA: f32 =  1.0;
}

impl Application for Lienzo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Lienzo, Command<Message>) {
        (
            Lienzo {
                last: Vec::new(),
                exit: button::State::default(),
                enabled: false,
                should_exit: false,
                grid: Grid::default(),
                rng: rand::thread_rng(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Simple Circle")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick(_) => (),
            Message::EventOccurred(event) if self.enabled => {
                match event {
                    iced_native::event::Event::Keyboard(keyboard::Event::CharacterReceived('j')) => {
                        println!("Jdayo");
                        self.grid.set_mino(Some(Minos::MinoO(mino::O::default())));
                        match self.grid.next {
                            Some(Minos::MinoO(_)) => println!("O in update"),
                            Some(Minos::MinoT(_)) => println!("T in update"),
                            _ => (),
                        }
                        
                    }
                    _ => {}
                }
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
            Message::Toggled(enabled) => {
                self.enabled = enabled;
            }
            Message::Exit => {
                self.should_exit = true;
            }

        }

        // mino::update(&self.grid.colors, &mut self.grid.next, &self.grid);

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        println!("ここがupdateか？？");
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            // time::every(std::time::Duration::from_millis(10)).map(Message::Tick)
            time::every(std::time::Duration::from_millis(10))
                .map(|_| Message::Tick(chrono::Local::now()))
        } else {
            // event listening...
            iced_native::subscription::events().map(Message::EventOccurred)
        }
        
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


        self.grid.colors[0][5] = 1;
        self.grid.colors[0][7] = 2;
        match self.grid.next { // grid.nextの中身にあるときはそれを優先(これはtest用)
            None => self.grid.set_mino(Some(mino::get_default_mino("T"))),
            _ => true,
        };
        // clone しないと,selfの変数は所有権のせいでmoveできない
        let canvas: Canvas<Message, Grid> = Canvas::new(
            Grid {
                // TODO: check which is better, self.grid.clone() or this below
                colors: self.grid.colors.clone(),
                next: self.grid.next.clone(),
                ..Default::default()
            }
        )
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

#[derive(Debug, Clone)]
struct Grid {
    square_size: f32,
    colors: Vec<Vec<usize>>, //row * column ; 列Vec<行Vec<>>
    pos: Option<Point>,
    next: Option<Minos>,
}

impl<Message> canvas::Program<Message> for Grid {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(bounds.size());

        let width = self.square_size * self.colors.len() as f32; //横の長さ 10
        let height = self.square_size * self.colors[0].len() as f32;  //縦の長さ 20
        let pos = match self.pos {
            None => {
                Point {x: (frame.width() - width) / 2.0, y: (frame.height() - height) / 2.0}
            }
            Some(pos) => pos,
        };
        frame = self.draw(frame, pos);

        frame = self.write_mino(frame);

        vec![frame.into_geometry()]
    }
}


impl Grid { 
    const COLOR_I: Color = Color {r: 0.0 / 255.0, g: 255.0 / 255.0, b: 255.0 / 255.0, a: 1.0}; // / 255.0
    const COLOR_O: Color = Color {r: 255.0 / 255.0, g: 255.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
    const COLOR_L: Color = Color {r: 243.0 / 255.0, g: 152.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
    const COLOR_J: Color = Color {r: 0.0 / 255.0, g: 0.0 / 255.0, b: 255.0 / 255.0, a: 1.0};
    const COLOR_S: Color = Color {r: 0.0 / 255.0, g: 255.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
    const COLOR_Z: Color = Color {r: 255.0 / 255.0, g: 0.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
    const COLOR_T: Color = Color {r: 148.0 / 255.0, g: 87.0 / 255.0, b: 164.0 / 255.0, a: 1.0};

    const COLOR_BACK: Color = Color {r: 181.0, g: 181.0, b: 181.0, a: 1.0};

    pub fn draw(&self, mut frame: Frame, point: Point) -> Frame {
        let mut x = point.x;
        let mut y;

        let len = self.colors[0].len();
        for column_c in self.colors.iter() { //列xの数forがまわる
            y = point.y;
            for (j, c) in (0..).zip(column_c.iter()) { //行yの数forがまわる
                if len - j > 21 { // index+5から開始
                    continue; // 20以上の盤面はかかない 21は半分かく
                }
                let pos_back ;
                let size_back;
                let pos;
                let size;
                if len - j == 21 { //一番上の段の場合半分のみ表示
                    pos_back = Point {x: x, y: y + self.square_size / 2.0};
                    size_back = Size {width: self.square_size, height: self.square_size / 2.0};
                    pos = Point {x: x + 1.0, y: y - 1.0 + self.square_size / 2.0};
                    size = Size {width: self.square_size - 1.0, height: self.square_size / 2.0 - 1.0};
                } else {
                    pos_back = Point {x: x, y: y};
                    size_back = Size {width: self.square_size, height: self.square_size};
                    pos = Point {x: x + 1.0, y: y - 1.0};
                    size = Size {width: self.square_size - 1.0, height: self.square_size - 1.0};
                };
                let square_back = canvas::Path::rectangle(pos_back, size_back);
                frame.fill(&square_back, Self::COLOR_BACK);

                let square = canvas::Path::rectangle(pos, size);
                frame.fill(&square , Self::get_color(*c));

                y += self.square_size;
            }
            x += self.square_size;
        }

        frame
    }

    pub fn write_mino(&self, mut frame: Frame) -> Frame {
        let next = match &self.next {
            None => return frame,
            Some(mino) => mino,
        };

        match next {
            Minos::MinoO(_) => println!("O処理"),
            Minos::MinoT(_) => println!("T処理"),
            _ => (),
        }

        frame = match next {
            Minos::MinoI(min) => self._write(frame, min.get_shape::<mino::I>(), min.get_position()),
            Minos::MinoJ(min) => self._write(frame, min.get_shape::<mino::J>(), min.get_position()),
            Minos::MinoL(min) => self._write(frame, min.get_shape::<mino::L>(), min.get_position()),
            Minos::MinoO(min) => self._write(frame, min.get_shape::<mino::O>(), min.get_position()),
            Minos::MinoS(min) => self._write(frame, min.get_shape::<mino::S>(), min.get_position()),
            Minos::MinoT(min) => self._write(frame, min.get_shape::<mino::T>(), min.get_position()),
            Minos::MinoZ(min) => self._write(frame, min.get_shape::<mino::Z>(), min.get_position()),
        };

        frame
    }

    fn _write(&self, mut frame: Frame, shape: [[usize; 4]; 4], start_point: Point) -> Frame {
        let _x = self.pos.unwrap().x + self.square_size * start_point.x;
        let mut y = self.pos.unwrap().y + self.square_size * start_point.y;

        let mut x;
        for i in 0..4 { //列xの数forがまわる
            x = _x;
            for j in 0..4 { //行yの数forがまわる
                let c = shape[i][j];
                if c == 0 { //minoでないマスは書かない
                    x += self.square_size; // TがJになるbugの修正
                    continue;
                }
                let pos_back ;
                let size_back;
                let pos;
                let size;
                if start_point.y + (i as f32) == 0.0 { //一番上の段の場合半分のみ表示
                    pos_back = Point {x: x, y: y + self.square_size / 2.0};
                    size_back = Size {width: self.square_size, height: self.square_size / 2.0};
                    pos = Point {x: x + 1.0, y: y - 1.0 + self.square_size / 2.0};
                    size = Size {width: self.square_size - 1.0, height: self.square_size / 2.0 - 1.0};
                } else {
                    pos_back = Point {x: x, y: y};
                    size_back = Size {width: self.square_size, height: self.square_size};
                    pos = Point {x: x + 1.0, y: y - 1.0};
                    size = Size {width: self.square_size - 1.0, height: self.square_size - 1.0};
                };

                let square_back = canvas::Path::rectangle(pos_back, size_back);
                frame.fill(&square_back, Self::COLOR_BACK);

                let square = canvas::Path::rectangle(pos, size);
                frame.fill(&square , Self::get_color(c));

                //jはx軸なのでjが変わるごとにxを増やす
                x += self.square_size;
            }
            y += self.square_size;
        }
        frame
    }

    pub fn set_mino(&mut self, mino: Option<Minos>) -> bool {
        return match mino {
                None => false,
                Some(mino) => {
                    self.next = Some(mino);
                    true
                }
            }
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

    pub fn get_colors(&self) -> &Vec<Vec<usize>> {
        &self.colors
    }
}


impl std::default::Default for Grid {
    fn default() -> Self {
        let column_num = 25; //20まで見える
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
            pos: Some(Point {x: 284.0, y: 62.5}),
            next: None,
        }
    }
}
