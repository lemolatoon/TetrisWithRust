mod tetris_core;
mod grid;

use iced::futures::future::Map;
use iced::keyboard::KeyCode;
use iced::keyboard::Modifiers;
use iced::time;
use rand::Rng;

use tetris_core::tetris_core::mino::Mino;
use tetris_core::tetris_core::mino::Minos;
use tetris_core::tetris_core::mino;

use grid::grid::grids::Grid;

use chrono;

use iced::{Align, Application, Button, Canvas, Checkbox, Clipboard, Color, Column, Command, Container, Element, HorizontalAlignment, Length, Point, Rectangle, Row, Settings, Size, Text, button, canvas::{self, Frame}, executor, keyboard};

use iced_native::event::Event::Keyboard;
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::Event::KeyReleased;

use iced_native;



use mino::get_mino_sets;

pub fn main() {
    let a = grid::grid::grids::Grid::default();
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

    now_drop: chrono::DateTime<chrono::Local>,
    soft_drop_flag: bool,

    now_right_left: chrono::DateTime<chrono::Local>,
    right_flag: bool,
    left_flag: bool,
    right_left_done_flag: bool,
    high_speed_right_left_flag: bool,

    now_placement: chrono::DateTime<chrono::Local>,
    place_flag: bool,
    place_cancel_count: usize,
}


#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    Tick(chrono::DateTime<chrono::Local>),
    Toggled(bool),
    Exit,
}


impl Lienzo {
    // mili秒
    const NATURAL_DROP_DELTA: i64 =  1000; // 1 * 1000
    const SOFT_DROP_DELTA: i64 = 50; // 1 / 20 * 1000 (softdropは二十倍の速度)

    const WAIT_TIME_LEFT_RIGHT: i64 = 183; // 0.3秒たつまではブロック一つのみ
    const RIGHT_LEFT_DELTA: i64 = 3;

    const PLACEMENT_LOCK_DOWN_DELTA: i64 = 500;

    const MODIFIER: Modifiers = Modifiers {shift: false, control: false, alt: false, logo: false};

    fn drop_check(&mut self, now: chrono::DateTime<chrono::Local>) -> bool { //障害物により下がれなかった場合のみfalse
        if !self.soft_drop_flag { // natural drop ing...(not soft drop)
            if now.timestamp_millis() - self.now_drop.timestamp_millis() > Self::NATURAL_DROP_DELTA { //一秒ごとに
                self.now_drop = now; //基準をリセット(0から数え直し)
                return self.grid.next.drop(&self.grid.colors);
            }
        } else { // start soft drop
            if now.timestamp_millis() - self.now_drop.timestamp_millis() > Self::SOFT_DROP_DELTA { // 1 / 20 秒ごとに
                self.now_drop = now; //基準をリセット(0から数え直し)
                return self.grid.next.drop(&self.grid.colors);

            }
        }
        true
    }

    fn right_left_check(&mut self, now: chrono::DateTime<chrono::Local>) { //右平行移動
        if self.right_flag { // waiting 0.3 second
            if !self.right_left_done_flag { //まだ最初の１ブロックも動かしてないならば
                self.high_speed_right_left_flag = false; //high speed modeをリセット
                self.grid.next.right(&self.grid.colors);
                self.now_right_left = now;
            } else if self.high_speed_right_left_flag && now.timestamp_millis() - self.now_right_left.timestamp_millis() > Self::RIGHT_LEFT_DELTA { //高速移動
                self.grid.next.right(&self.grid.colors);
                self.high_speed_right_left_flag = true;
            } else if now.timestamp_millis() - self.now_right_left.timestamp_millis() > Self::WAIT_TIME_LEFT_RIGHT { // 0.3s経過したならば高速モードに移行
                self.grid.next.right(&self.grid.colors);
                self.high_speed_right_left_flag = true;
            }
        } else if self.left_flag {
            if !self.right_left_done_flag { //まだ最初の１ブロックも動かしてないならば
                self.high_speed_right_left_flag = false; //high speed modeをリセット
                self.grid.next.left(&self.grid.colors);
                self.now_right_left = now;
            } else if self.high_speed_right_left_flag && now.timestamp_millis() - self.now_right_left.timestamp_millis() > Self::RIGHT_LEFT_DELTA { //高速移動
                self.grid.next.left(&self.grid.colors);
                self.high_speed_right_left_flag = true;
            } else if now.timestamp_millis() - self.now_right_left.timestamp_millis() > Self::WAIT_TIME_LEFT_RIGHT { // 0.3s経過したならば高速モードに移行
                self.grid.next.left(&self.grid.colors);
                self.high_speed_right_left_flag = true;
            }
        }
    }

    fn place_check(&mut self, now: chrono::DateTime<chrono::Local>) {
        if !self.place_flag{ //flagない場合時計のみ更新
            self.now_placement = now;
        } else if now.timestamp_millis() - self.now_placement.timestamp_millis() > Self::PLACEMENT_LOCK_DOWN_DELTA {
            self.grid.next.place(&mut self.grid.colors);
            self.grid.next = self.grid.get_mino();
            self.now_placement = now;
        } //flag: trueだが待っている状態
    }

    fn hard_drop(&mut self) {
        while self.grid.next.drop(&self.grid.colors) { //落ちてる間はtrue
        }
        self.grid.next.place(&mut self.grid.colors);
    }


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

                now_drop: chrono::Local::now(),
                soft_drop_flag: false,

                now_right_left: chrono::Local::now(),
                right_flag: false,
                left_flag: false,
                right_left_done_flag: false,
                high_speed_right_left_flag: false,

                now_placement: chrono::Local::now(),
                place_flag: true, //とりあえずつねにtrue
                place_cancel_count: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Simple Circle")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                // SoftDropなどの処理
                if !self.drop_check(local_time) {
                    self.place_check(local_time);
                }
                self.right_left_check(local_time);
                println!("Position: {:?}", self.grid.next.get_position());
            },
            Message::EventOccurred(event) if self.enabled => {
                match event {
                    Keyboard(keyboard::Event::CharacterReceived('c')) => {
                        println!("Cdayo");
                        self.grid.next = self.grid.get_mino();
                    },
                    Keyboard(KeyPressed {key_code: KeyCode::S, modifiers: Self::MODIFIER}) => self.soft_drop_flag = true,
                    Keyboard(KeyReleased {key_code: KeyCode::S, modifiers: Self::MODIFIER}) => self.soft_drop_flag = false,
                    Keyboard(KeyPressed {key_code: KeyCode::D, modifiers: Self::MODIFIER}) => self.right_flag = true,
                    Keyboard(KeyReleased {key_code: KeyCode::D, modifiers: Self::MODIFIER}) => self.right_flag = false,
                    Keyboard(KeyPressed {key_code: KeyCode::A, modifiers: Self::MODIFIER}) => self.left_flag = true,
                    Keyboard(KeyReleased {key_code: KeyCode::A, modifiers: Self::MODIFIER}) => self.left_flag = false,
                    Keyboard(KeyPressed {key_code: KeyCode::J, modifiers: Self::MODIFIER}) => {self.grid.next.rotate_left(&self.grid.colors);()},
                    Keyboard(KeyPressed {key_code: KeyCode::K, modifiers: Self::MODIFIER}) => {self.grid.next.rotate_right(&self.grid.colors);()},
                    Keyboard(KeyPressed { key_code: KeyCode::W, modifiers: Self::MODIFIER}) => self.hard_drop(),
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

        // TODO: このままでは、eventを受けたときに、なぜかsubscription loopが止まってしまう

        // 50 mili秒ごとに呼ばれる
        let tick = time::every(std::time::Duration::from_millis(50))
                .map(|_| Message::Tick(chrono::Local::now()));

        // events毎に呼ばれる
        let events = iced_native::subscription::events().map(Message::EventOccurred);

        // 複数のsubscriptionを渡したいときにはvecにいれてbatchに渡そう!!
        iced_futures::Subscription::batch(vec![tick, events])

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


        // ↓とりあえずなにか表示したい時用
        // self.grid.next = self.grid.get_mino();
        // clone しないと,selfの変数は所有権のせいでmoveできない
        let mut grid = Grid::default();
        grid.colors = self.grid.colors.clone();
        grid.next = self.grid.next.clone();
        let canvas: Canvas<Message, Grid> = Canvas::new(
            // TODO: check which is better, self.grid.clone() or this below
            grid
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
