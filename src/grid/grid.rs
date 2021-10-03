
/// There starts the definition of Grid
pub mod grids {
    use crate::tetris_core;
    use iced::{Point, canvas, Rectangle, Color, Size};
    use iced::canvas::Frame;

    use tetris_core::tetris_core::mino;
    use mino::Minos;
    use mino::Mino; //trait
    use mino::get_mino_sets;

    #[derive(Debug, Clone)]
    pub struct Grid {
        square_size: f32,
        pub colors: Vec<Vec<usize>>, //row * column ; 列Vec<行Vec<>>
        pos: Option<Point>,
        pub next: Minos, 
        pub effect_lines: Vec<usize>,

        mino_sets: Vec<Minos>,
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

            frame = self.write_effect(frame, pos);

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
            frame = match &self.next {
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
            // 21マス目を追加したことによる補正(y + 1)
            let mut y = self.pos.unwrap().y + self.square_size * (start_point.y + 1.0);

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
                    if start_point.y + (i as f32) == - 1.0 { //一番上の段の場合半分のみ表示
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

        fn write_effect(&self, mut frame: Frame, pos: Point) -> Frame {
            // effect drawing
        
            // println!("self.effect_lines: {:?}", &self.effect_lines);
            let x0 = pos.x;
            let y0 = pos.y;

            let mut x = x0;
            let mut y = y0;
            for index in self.effect_lines.iter() {
                // 21マスまで設定してるので 25-21=4 マス分ずらす
                y = y0 + (*index - 4) as f32 * self.square_size;
                for i in 0..self.colors.len() {
                    x = x0 + i as f32 * self.square_size;

                    let pos_back ;
                    let size_back;
                    let pos;
                    let size;
                    pos_back = Point {x: x, y: y};
                    size_back = Size {width: self.square_size, height: self.square_size};
                    pos = Point {x: x + 1.0, y: y - 1.0};
                    size = Size {width: self.square_size - 1.0, height: self.square_size - 1.0};
                    let square_back = canvas::Path::rectangle(pos_back, size_back);
                    frame.fill(&square_back, Self::COLOR_BACK);

                    let square = canvas::Path::rectangle(pos, size);
                    frame.fill(&square , Color::BLACK);
                }
            }
            frame
        }

        pub fn set_effect_lines(&mut self, lines: Vec<usize>) {
            self.effect_lines = lines;
        }

        pub fn clear_effect_lines(&mut self) {
            self.effect_lines = Vec::new();
        }

        pub fn set_mino(&mut self, mino: Minos) {
                self.next = mino;
        }

        pub fn get_mino(&mut self) -> Minos { //&mut selfにしたらなぜか怒られなくなった
            // 常に二周分保持し、minoが７以下になったら、補充する
            if self.mino_sets.len() < 8 {
                    let mut tmp = get_mino_sets();
                    tmp.append(&mut self.mino_sets);
                    self.mino_sets = tmp;
                    self.mino_sets.pop().unwrap()
            } else {
                self.mino_sets.pop().unwrap()
            }

            /*
            match &self.mino_sets.pop() {
                None => { // emptyの場合
                    self.mino_sets = get_mino_sets();
                    self.mino_sets.pop().unwrap()
                },
                Some(m) => m,
            }
            */
        }

        pub fn effect_lines(&mut self, indexes: &Vec<usize>, color: usize) {
            // make sure 0 < index < 24
            for column in &mut self.colors {
                for index in indexes.iter() {
                    column[*index] = color;
                }
            }
        }

        fn get_color(i: usize) -> Color {
            return if i == 0 {
                Color::from_rgb8(232, 232, 232) // background
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
            } else if i == 8 {
                Color::BLACK
            } else if i == 9 {
                Color::WHITE
            } else {
                panic!("illegal color number: {}", i);
            };
        }

        pub fn get_next(&self, i: usize) -> &Minos {
            // make sure 0 < i < 6
            let len = self.mino_sets.len();
            return &self.mino_sets[len - i - 1];
        }

    }

    impl std::default::Default for Grid {
        fn default() -> Self { 
            // the image of board is below
            //   |      
            //___|___________>x
            //   |         10
            //   |
            //   |
            //   |
            //   |
            //   |
            //   |25
            //   \/
            //   y
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

            let mut mino_sets = get_mino_sets();

            Self {
                square_size: 20.0,
                colors: colors,
                pos: Some(Point {x: 284.0, y: 62.5}),
                next: mino_sets.pop().unwrap(),
                effect_lines: Vec::new(),
                mino_sets: mino_sets,
            }
        }
    }


    #[derive(Debug, Clone)]
    pub struct GridMino {
        square_size: f32,
        pub colors: Vec<Vec<usize>>, //row * column ; 列Vec<行Vec<>>
        pos: Point,
        pub next: isize,
    }


    impl<Message> canvas::Program<Message> for GridMino {
        fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
            let mut frame = Frame::new(bounds.size());

            let width = self.square_size * self.colors.len() as f32; //横の長さ 10
            let height = self.square_size * self.colors[0].len() as f32;  //縦の長さ 20
            let pos;
            if self.pos.y == 0.0 {
                pos = Point {x: (frame.width() - width) / 2.0, y: (frame.height() - height) / 2.0};
                println!("{:?}", pos);
            } else {
                pos = self.pos;
            }
            frame = self.draw(frame, pos);

            frame = self.write_mino(frame);

            vec![frame.into_geometry()]
        }
    }


    impl GridMino { 
        const COLOR_I: Color = Color {r: 0.0 / 255.0, g: 255.0 / 255.0, b: 255.0 / 255.0, a: 1.0}; // / 255.0
        const COLOR_O: Color = Color {r: 255.0 / 255.0, g: 255.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
        const COLOR_L: Color = Color {r: 243.0 / 255.0, g: 152.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
        const COLOR_J: Color = Color {r: 0.0 / 255.0, g: 0.0 / 255.0, b: 255.0 / 255.0, a: 1.0};
        const COLOR_S: Color = Color {r: 0.0 / 255.0, g: 255.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
        const COLOR_Z: Color = Color {r: 255.0 / 255.0, g: 0.0 / 255.0, b: 0.0 / 255.0, a: 1.0};
        const COLOR_T: Color = Color {r: 148.0 / 255.0, g: 87.0 / 255.0, b: 164.0 / 255.0, a: 1.0};

        const COLOR_BACK: Color = Color {r: 181.0, g: 181.0, b: 181.0, a: 1.0};

        pub fn default(size: f32, hold_num: isize, pos: Point) -> GridMino {
            let mut colors = Vec::with_capacity(4);
            for _ in 0..4 {
                colors.push(vec![0, 0, 0, 0]);
            }

            Self {
                square_size: size,
                colors: colors,
                pos: pos,
                next: hold_num,
            }
        }

        pub fn draw(&self, mut frame: Frame, point: Point) -> Frame {
            let mut x = point.x;
            let mut y;

            let len = self.colors[0].len();
            for column_c in self.colors.iter() { //列xの数forがまわる
                y = point.y;
                for (j, c) in (0..).zip(column_c.iter()) { //行yの数forがまわる
                    let pos_back ;
                    let size_back;
                    let pos;
                    let size;
                    pos_back = Point {x: x, y: y};
                    size_back = Size {width: self.square_size, height: self.square_size};
                    pos = Point {x: x + 1.0, y: y - 1.0};
                    size = Size {width: self.square_size - 1.0, height: self.square_size - 1.0};
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
            if self.next == -1 { // -1 ならなにも書かない
                return frame;
            }
            frame = match Minos::num2mino(self.next) {
                Minos::MinoI(min) => self._write(frame, min.get_shape::<mino::I>(), Point {x: 0.0, y: 0.0}),
                Minos::MinoJ(min) => self._write(frame, min.get_shape::<mino::J>(), Point {x: 0.0, y: 0.0}),
                Minos::MinoL(min) => self._write(frame, min.get_shape::<mino::L>(), Point {x: 0.0, y: 0.0}),
                Minos::MinoO(min) => self._write(frame, min.get_shape::<mino::O>(), Point {x: 0.0, y: 0.0}),
                Minos::MinoS(min) => self._write(frame, min.get_shape::<mino::S>(), Point {x: 0.0, y: 0.0}),
                Minos::MinoT(min) => self._write(frame, min.get_shape::<mino::T>(), Point {x: 0.0, y: 0.0}),
                Minos::MinoZ(min) => self._write(frame, min.get_shape::<mino::Z>(), Point {x: 0.0, y: 0.0}),
            };

            frame
        }

        fn _write(&self, mut frame: Frame, shape: [[usize; 4]; 4], start_point: Point) -> Frame {
            let _x = self.pos.x + self.square_size * start_point.x;
            // 21マス目を追加したことによる補正(y + 1)
            let mut y = self.pos.y + self.square_size * (start_point.y);

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
                    pos_back = Point {x: x, y: y};
                    size_back = Size {width: self.square_size, height: self.square_size};
                    pos = Point {x: x + 1.0, y: y - 1.0}; // changed
                    size = Size {width: self.square_size - 1.0, height: self.square_size - 1.0};

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
}