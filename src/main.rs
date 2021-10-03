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
use grid::grid::grids::GridMino;

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

    hold: isize,

    // ミノ落下時の制御
    now_drop: chrono::DateTime<chrono::Local>,
    soft_drop_flag: bool,
    hard_drop_flag: bool,

    // ミノ左右移動時の制御
    now_right_left: chrono::DateTime<chrono::Local>,
    right_flag: bool,
    left_flag: bool,
    right_left_done_flag: bool,
    high_speed_right_left_flag: bool,

    // ミノ設置時の制御
    now_placement: chrono::DateTime<chrono::Local>,
    place_flag: bool,
    place_cancel_count: usize,

    // ライン消去時の制御
    now_erase: chrono::DateTime<chrono::Local>,
    erase_flag: bool,
    erased_lines: Vec<usize>,
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

    const WAIT_TIME_LEFT_RIGHT: i64 = 500; // 0.3秒たつまではブロック一つのみ (origin 183)
    const RIGHT_LEFT_DELTA: i64 = 3;

    const PLACEMENT_LOCK_DOWN_DELTA: i64 = 500;

    const LINES_ERASE_DELTA: i64 = 1000;

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
        let droppable = self.grid.next.droppable(&self.grid.colors); // dropできるのかどうか
        if droppable {
            self.now_placement = now;
        } else if now.timestamp_millis() - self.now_placement.timestamp_millis() > Self::PLACEMENT_LOCK_DOWN_DELTA {
            // droppable かつ　所定時間以上経過
            self.place_flag = true;
            if self.effect_init_place(now) {
                // もし消すラインがなくeffectもないのなら
                self.next_mino();
            }

            self.now_placement = now;
        }
    }

    fn effect_init_place(&mut self, now: chrono::DateTime<chrono::Local>) -> bool {
            // 消す予定のライン
            self.grid.next._place(&mut self.grid.colors);
            self.erased_lines = self.grid.next.erase_lines(&mut self.grid.colors);
            
            if !self.erased_lines.is_empty() {
                //init
                self.erase_flag = true;
                self.now_erase = now;
                // TODO: call fn here : write effect in grid.rs
                // the fn is also not implemented
                // self.grid.set_effect_lines(self.erased_lines.clone());
                self.grid.effect_lines = self.erased_lines.clone();
                self.effect_check(now);
                return false;
            } else {
                // どこも消さないならなにもしない
                return true;
            }
    }

    fn effect_check(&mut self, now: chrono::DateTime<chrono::Local>) {
        // erase が確定したときに呼ばれる
        // つまり、place_check内部からも呼ばれる
        // 戻り値はミノを更新すべきかどうか呼ばれる
        // つまり !effectの有無
        if now.timestamp_millis() - self.now_erase.timestamp_millis() > Self::LINES_ERASE_DELTA {
            // effect終了
            self.erase_flag = false;
            self.erased_lines = Vec::new();
            self.grid.clear_effect_lines();
            self.grid.next.erase(&mut self.grid.colors);

            self.next_mino();
            // TODO: clear the Vector in grid which is for effect
        } else {
            // self.grid.set_effect_lines(self.erased_lines);
            self.grid.effect_lines = self.erased_lines.clone();
        }
    }

    fn next_mino(&mut self) {
        // effectから次のminoへうつるときに呼ばれる
        self.grid.next = self.grid.get_mino();
        self.place_flag = false;
    }

    fn hard_drop(&mut self, now: chrono::DateTime<chrono::Local>) {
        if !self.place_flag { // place処理中でないのなら
            self.place_flag = true;
            while self.grid.next.drop(&self.grid.colors) { //落ちてる間はtrue
            }
            if self.effect_init_place(now) {
                self.next_mino();
            }
        }
        self.hard_drop_flag = false;
    }

    fn hold(&mut self) {
        if self.hold == -1 { //minoなし
            self.hold = Minos::mino2num(&self.grid.next);
            self.grid.next = self.grid.get_mino();
        } else {
            let tmp = self.hold;
            self.hold = Minos::mino2num(&self.grid.next);
            self.grid.next = Minos::num2mino(tmp);
        }
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

                hold: -1,

                // ミノ落下時の制御
                now_drop: chrono::Local::now(),
                soft_drop_flag: false,
                hard_drop_flag: false,

                // ミノ左右移動時の制御
                now_right_left: chrono::Local::now(),
                right_flag: false,
                left_flag: false,
                right_left_done_flag: false,
                high_speed_right_left_flag: false,

                // ミノ設置時の制御
                now_placement: chrono::Local::now(),
                place_flag: false, // for not to interrapted
                place_cancel_count: 0,

                // ライン消去時の制御
                now_erase: chrono::Local::now(),
                erase_flag: false,
                erased_lines: Vec::new(),
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
                if self.erase_flag { // 消去処理中なら
                    self.effect_check(local_time);
                } else if !self.place_flag { //邪魔しちゃだめ
                    if !self.hard_drop_flag {
                        // SoftDropなどの処理
                        self.drop_check(local_time);
                        self.place_check(local_time);
                        self.right_left_check(local_time);
                    } else {
                        self.hard_drop(local_time);
                    }
                } else {
                }
            },
            Message::EventOccurred(event) => {
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
                    Keyboard(KeyPressed {key_code: KeyCode::J, modifiers: Self::MODIFIER}) => {self.grid.next.SRS_rotate_left(&self.grid.colors);()},
                    Keyboard(KeyPressed {key_code: KeyCode::K, modifiers: Self::MODIFIER}) => {self.grid.next.SRS_rotate_right(&self.grid.colors);()},
                    Keyboard(KeyPressed { key_code: KeyCode::W, modifiers: Self::MODIFIER}) => self.hard_drop_flag = true,
                    Keyboard(KeyPressed { key_code: KeyCode::L, modifiers: Self::MODIFIER}) => self.hold(),
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
        // let events = self.last.iter().fold( 
            // Column::new().spacing(10),
            // |column, event| {
                // column.push(Text::new(format!("{:?}", event)).size(20))
            // },
        // );

        // let toggle = Checkbox::new(
        //    self.enabled, //checkboxにより変化するflag
        //    "Listen to runtime events",
        //    Message::Toggled,
        //);

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
        // NOTE: This call needs to satisfy syn
        grid.effect_lines = self.grid.effect_lines.clone();
        let canvas: Canvas<Message, Grid> = Canvas::new(
            // TODO: check which is better, self.grid.clone() or this below
            grid
        )
            .width(Length::Units(768))
            .height(Length::Units(525));


        let hold_canvas: Canvas<Message, GridMino> = Canvas::new(
            GridMino::default(20.0, self.hold, Point {x: 2.5, y: 2.5})
        )
            .width(Length::Units(85))
            .height(Length::Units(85));

        
        let next0_canvas: Canvas<Message, GridMino> = Canvas::new(
            GridMino::default(30.0, Minos::mino2num(&self.grid.get_next(0)), Point{x: 2.5, y: 2.5})
        )
            .width(Length::Units(125))
            .height(Length::Units(125));

        let next_num: usize = 4;
        let mut nexts_canvas: Vec<Canvas<Message, GridMino>> = Vec::with_capacity(4);
        for i in 0..next_num {
            let next_canvas: Canvas<Message, GridMino> = Canvas::new(
                GridMino::default(20.0, Minos::mino2num(&self.grid.get_next(i+1)), Point{x: 2.5, y: 2.5})
            )
                .width(Length::Units(80))
                .height(Length::Units(80));
                nexts_canvas.push(next_canvas);
        }

        let nexts = Column::new()
            .align_items(Align::Center)
            .push(next0_canvas)
            .spacing(10)
            .push(nexts_canvas.pop().unwrap())
            .spacing(10)
            .push(nexts_canvas.pop().unwrap())
            .spacing(10)
            .push(nexts_canvas.pop().unwrap())
            .spacing(10)
            .push(nexts_canvas.pop().unwrap());

        // 関数型っぽくかけそうだけどよくわからない
        // nexts_canvas.iter()
            // .fold(nexts, |x, y| x.push(nexts_canvas.pop().unwrap()));



        //縦に積み重ねる
        let explanation = Column::new()
            .align_items(Align::Center)
            .spacing(100)
            // .push(events)
            // .push(toggle)
            .push(exit);

        let content = Row::new()
            .align_items(Align::Start)
            .push(hold_canvas)
            .push(canvas)
            .push(nexts)
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
