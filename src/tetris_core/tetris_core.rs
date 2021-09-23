
pub mod mino {
    use iced::Point;
    use std::default::Default;
    // use std::{sync::mpsc, thread};

    #[derive(Debug, PartialEq, Clone)]
    pub enum State {
        State0,
        State1,
        State2,
        State3,
    }

    pub trait MinoShape {
        const SHAPE0: [[usize; 4]; 4];
        const SHAPE1: [[usize; 4]; 4];
        const SHAPE2: [[usize; 4]; 4];
        const SHAPE3: [[usize; 4]; 4];
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Minos {
        MinoI(I),
        MinoJ(J),
        MinoL(L),
        MinoO(O),
        MinoS(S),
        MinoT(T),
        MinoZ(Z),
    }


    impl Minos {
        pub fn drop(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self.shift(0, 1);
            if !self.is_settable(board) {
                self.shift(0, -1);
                return false;
            }
            true
        }

        pub fn erase_lines(&mut self, board: &Vec<Vec<usize>>) -> Vec<usize> {
            let columns_num = board.len(); //列 = 10
            let rows_num = board[0].len(); //行 = 25
            let mut erasable_vec = Vec::new();
            for j in 0..rows_num { //行ごとにcheck
                let mut erasable = true;
                for i in 0..columns_num {
                    if board[i][j] == 0 {
                        erasable = false;
                    } //穴があるなら消せない
                }
                if erasable { //この行がerasableなら行番号をvectorに追加
                    erasable_vec.push(j);
                }
            } 
            erasable_vec
        }

        pub fn hard_drop(&mut self, board: &mut Vec<Vec<usize>>) {
            while self.drop(board) {} //落ちられるところまで落ちる
            self.place(board);
        }

        pub fn place(&mut self, board: &mut Vec<Vec<usize>>) -> bool {
            self._place(board);
            println!("place");
            self.erase(board);
            true
        }

        pub fn erase(&mut self, board: &mut Vec<Vec<usize>>) {
            println!("====erase_start======");
            let erasable_vec = self.erase_lines(board);
            println!("plan to erase: {:?}", erasable_vec);
            for j in erasable_vec {
                // 上からminoがshiftしてくるだけなので
                // 行番号はそのまま使える
                for column in board.iter() {
                    println!("{:?}", column);
                }
                Self::_erase_row(j, board);
            }
            println!("====erase_end========");
        }

        pub fn right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self.shift(1, 0);
            if !self.is_settable(board) {
                self.shift(-1, 0);
                return false;
            }
            true
        }

        pub fn left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self.shift(-1, 0);
            if !self.is_settable(board) {
                self.shift(1, 0);
                return false;
            }
            true
        }

        pub fn rotate_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if !self.is_settable(board) {
                self._rotate_left();
                return false;
            }
            true
        }

        pub fn rotate_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if !self.is_settable(board) {
                self._rotate_right();
                return false;
            }
            true
        }


        fn _erase_row(index: usize, board: &mut Vec<Vec<usize>>) {
            // 列ごとに取り出し特定の行番号(index)を消去する
            println!("remove: {}", index);
            for column in board { // NOTICE: here exsits borrow 
                column.remove(index);
                let mut vec_tmp = vec![0];
                vec_tmp.append(column); //前(盤面上)から0を結合
                *column = vec_tmp;
                // 値を消した分、うえから０をいれる
            }
        }


        pub fn is_settable(&mut self, board: &Vec<Vec<usize>>) -> bool {
            let pos = self.get_position();
            let x = pos.x as isize;
            let y = pos.y as isize;
            let shape = self.get_shape();
            for i in 0..4 { // 各mino型について検証
                for j in 0..4 {
                    let i2 = i as isize;
                    let j2 = j as isize;
                    /*
                    println!("(x + (i as isize)) as usize = {}", (x + (i as isize)) as usize);
                    println!("y: {}, j: {}", y, j);
                    println!("(y + (j as isize)) as usize + 5 = {}", (y + (j as isize) + 5) as usize);
                    */
                    if shape[j][i] != 0 && ( // shapeにアクセスするときは (j, i) で行う
                        //ここの条件判定を抜けるとx > 0, y > 0が保証される(要検証)
                        x + i2 < 0 || x + i2 > 9 || y + j2 + 5 > 19 + 5 || // ここのやつは満たす(=マスからはみ出る)
                        //xをusizeに変換すると-1 -> INT_MAXとなるので気をつけよう
                        board[(x + (i as isize)) as usize][(y + (j as isize) + 5) as usize] != 0 // 上に隠れている５行分y座標たす
                    )
                    { //そのマスにミノがあり、board上にもあるなら
                        /*
                        println!("`is settable` returns false at (i, j): ({}, {})", i, j);
                        println!("y + j2 + 5 = {}", y + j2 + 5);
                        */
                        return false;
                    }
                }
            }
            true
        }

        pub fn SRS_rotate_right(&mut self, board: &Vec<Vec<usize>>) -> bool { // super rotation system
            match self {
                Minos::MinoI(_) => self._I_SRS_rotate_right(board), // Iミノのみ挙動が違う
                _ => self._SRS_rotate_right(board),
            }
        }

        pub fn SRS_rotate_left(&mut self, board: &Vec<Vec<usize>>) -> bool { // super rotation system
            match self {
                Minos::MinoI(_) => self._I_SRS_rotate_left(board), // Iミノのみ挙動が違う
                _ => self._SRS_rotate_left(board),
            }
        }

    }

    impl Minos {

        pub fn _SRS_rotate_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            match self.get_state() {
                State::State0 => self._SRS_s0_s1_right(board),
                State::State1 => self._SRS_s1_s2_right(board),
                State::State2 => self._SRS_s2_s3_right(board),
                State::State3 => self._SRS_s3_s0_right(board),
            }
        }

        pub fn _SRS_rotate_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            match self.get_state() {
                State::State0 => self._SRS_s0_s3_left(board),
                State::State1 => self._SRS_s1_s0_left(board),
                State::State2 => self._SRS_s2_s1_left(board),
                State::State3 => self._SRS_s3_s2_left(board),
            }
        }

        pub fn _I_SRS_rotate_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            match self.get_state() {
                State::State0 => self._I_SRS_s0_s1_right(board),
                State::State1 => self._I_SRS_s1_s2_right(board),
                State::State2 => self._I_SRS_s2_s3_right(board),
                State::State3 => self._I_SRS_s3_s0_right(board),
            }
        }
        
        pub fn _I_SRS_rotate_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            match self.get_state() {
                State::State0 => self._I_SRS_s0_s3_left(board),
                State::State1 => self._I_SRS_s1_s0_left(board),
                State::State2 => self._I_SRS_s2_s1_left(board),
                State::State3 => self._I_SRS_s3_s2_left(board),
            }
        }

        pub fn _SRS_s0_s1_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, -1);
            if self.is_settable(board) {return true;}

            self.shift(1, 3);
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(1, -2);
            self._rotate_left();
            false
        }

        pub fn _SRS_s1_s2_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, 1);
            if self.is_settable(board) {return true;}

            self.shift(-1, -3);
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(-1, 2);
            self._rotate_left();
            false
        }

        pub fn _SRS_s2_s3_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, -1);
            if self.is_settable(board) {return true;}

            self.shift(-1, 3);
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(-1, -2);
            self._rotate_left();
            false
        }

        pub fn _SRS_s3_s0_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, 1);
            if self.is_settable(board) {return true;}

            self.shift(1, -3);
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(1, 2);
            self._rotate_left();
            false
        }

        pub fn _SRS_s0_s3_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, -1);
            if self.is_settable(board) {return true;}

            self.shift(-1, 3);
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(-1, -2);
            self._rotate_right();
            false
        }

        pub fn _SRS_s1_s0_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, 1);
            if self.is_settable(board) {return true;}

            self.shift(-1, -3);
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(-1, 2);
            self._rotate_right();
            false
        }

        pub fn _SRS_s2_s1_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, -1);
            if self.is_settable(board) {return true;}

            self.shift(1, 3);
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(1, -2);
            self._rotate_right();
            false
        }

        pub fn _SRS_s3_s2_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, 1);
            if self.is_settable(board) {return true;}

            self.shift(1, -3);
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(1, 2);
            self._rotate_right();
            false
        }

        pub fn _I_SRS_s0_s1_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(-2, 0);
            if self.is_settable(board) {return true;}

            self.shift(3, 0);
            if self.is_settable(board) {return true;}

            self.shift(-3, 1);
            if self.is_settable(board) {return true;}

            self.shift(3, -3);
            if self.is_settable(board) {return true;}

            self.shift(-1, 2);
            self._rotate_left();
            false
        }

        pub fn _I_SRS_s1_s2_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(3, 0);
            if self.is_settable(board) {return true;}

            self.shift(-3, -2);
            if self.is_settable(board) {return true;}

            self.shift(3, 3);
            if self.is_settable(board) {return true;}

            self.shift(-2, -1);
            self._rotate_left();
            false
        }

        pub fn _I_SRS_s2_s3_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(2, 0);
            if self.is_settable(board) {return true;}

            self.shift(-3, 0);
            if self.is_settable(board) {return true;}

            self.shift(3, -1);
            if self.is_settable(board) {return true;}

            self.shift(-3, 3);
            if self.is_settable(board) {return true;}

            self.shift(1, -2);
            self._rotate_left();
            false
        }

        pub fn _I_SRS_s3_s0_right(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_right();
            if self.is_settable(board) {return true;}

            self.shift(-2, 0);
            if self.is_settable(board) {return true;}

            self.shift(3, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, 2);
            if self.is_settable(board) {return true;}

            self.shift(-3, -3);
            if self.is_settable(board) {return true;}

            self.shift(2, 1);
            self._rotate_left();
            false
        }

        pub fn _I_SRS_s0_s3_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(-1, 0);
            if self.is_settable(board) {return true;}

            self.shift(3, 0);
            if self.is_settable(board) {return true;}

            self.shift(-3, -2);
            if self.is_settable(board) {return true;}

            self.shift(3, 3);
            if self.is_settable(board) {return true;}

            self.shift(-2, -1);
            self._rotate_right();
            false
        }

        pub fn _I_SRS_s1_s0_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(2, 0);
            if self.is_settable(board) {return true;}

            self.shift(-3, 0);
            if self.is_settable(board) {return true;}

            self.shift(3, -1);
            if self.is_settable(board) {return true;}

            self.shift(-3, 3);
            if self.is_settable(board) {return true;}

            self.shift(1, -2);
            self._rotate_right();
            false
        }

        pub fn _I_SRS_s2_s1_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(-3, 0);
            if self.is_settable(board) {return true;}

            self.shift(3, 2);
            if self.is_settable(board) {return true;}

            self.shift(-3, -3);
            if self.is_settable(board) {return true;}

            self.shift(2, 1);
            self._rotate_right();
            false
        }

        pub fn _I_SRS_s3_s2_left(&mut self, board: &Vec<Vec<usize>>) -> bool {
            self._rotate_left();
            if self.is_settable(board) {return true;}

            self.shift(1, 0);
            if self.is_settable(board) {return true;}

            self.shift(-3, 0);
            if self.is_settable(board) {return true;}

            self.shift(0, 1);
            if self.is_settable(board) {return true;}

            self.shift(3, -3);
            if self.is_settable(board) {return true;}

            self.shift(-1, 2);
            self._rotate_right();
            false
        }

        pub fn get_state(&self) -> &State {
            match self {
                Minos::MinoI(min) => min.get_state(),
                Minos::MinoJ(min) => min.get_state(),
                Minos::MinoL(min) => min.get_state(),
                Minos::MinoO(min) => min.get_state(),
                Minos::MinoS(min) => min.get_state(),
                Minos::MinoT(min) => min.get_state(),
                Minos::MinoZ(min) => min.get_state(),
            }
        }


        pub fn mino2num(&self) -> isize {
            match self {
                Minos::MinoI(min) => 0,
                Minos::MinoJ(min) => 1,
                Minos::MinoL(min) => 2,
                Minos::MinoO(min) => 3,
                Minos::MinoS(min) => 4,
                Minos::MinoT(min) => 5,
                Minos::MinoZ(min) => 6,
            }
        }

        pub fn num2mino(num: isize) -> Minos {
            match num {
                0 => get_default_mino("I"),
                1 => get_default_mino("J"),
                2 => get_default_mino("L"),
                3 => get_default_mino("O"),
                4 => get_default_mino("S"),
                5 => get_default_mino("T"),
                6 => get_default_mino("Z"),
                _ => panic!("illegal number at `num2mino`"),
            }
        }

        pub fn get_position(&mut self) -> Point {
            match self {
                Minos::MinoI(min) => min.get_position(),
                Minos::MinoJ(min) => min.get_position(),
                Minos::MinoL(min) => min.get_position(),
                Minos::MinoO(min) => min.get_position(),
                Minos::MinoS(min) => min.get_position(),
                Minos::MinoT(min) => min.get_position(),
                Minos::MinoZ(min) => min.get_position(),
            }
        }

        pub fn _place(&mut self, board: &mut Vec<Vec<usize>>) {
            let pos = self.get_position();
            let x = pos.x as isize;
            let y = pos.y as isize;
            let shape = self.get_shape();
            for i in 0..4 {
                for j in 0..4 {
                    let c = shape[j][i];
                    if c != 0 { //minoがあるなら
                        board[(x + (i as isize)) as usize][(y + (j as isize) + 5) as usize] = shape[j][i];
                    }
                }
            }
        }

        pub fn set_position(&mut self, x: isize, y: isize) {
            match self {
                Minos::MinoI(min) => min.set_position(Point{x: x as f32, y: y as f32}),
                Minos::MinoJ(min) => min.set_position(Point{x: x as f32, y: y as f32}),
                Minos::MinoL(min) => min.set_position(Point{x: x as f32, y: y as f32}),
                Minos::MinoO(min) => min.set_position(Point{x: x as f32, y: y as f32}),
                Minos::MinoS(min) => min.set_position(Point{x: x as f32, y: y as f32}),
                Minos::MinoT(min) => min.set_position(Point{x: x as f32, y: y as f32}),
                Minos::MinoZ(min) => min.set_position(Point{x: x as f32, y: y as f32}),
            };
        }

        pub fn shift(&mut self, x: isize, y: isize) {
            match self {
                Minos::MinoI(min) => min._shift(x, y),
                Minos::MinoJ(min) => min._shift(x, y),
                Minos::MinoL(min) => min._shift(x, y),
                Minos::MinoO(min) => min._shift(x, y),
                Minos::MinoS(min) => min._shift(x, y),
                Minos::MinoT(min) => min._shift(x, y),
                Minos::MinoZ(min) => min._shift(x, y),
            };
        }

        pub fn _rotate_right(&mut self) {
            match self {
                Minos::MinoI(min) => min.rotate_right(),
                Minos::MinoJ(min) => min.rotate_right(),
                Minos::MinoL(min) => min.rotate_right(),
                Minos::MinoO(min) => min.rotate_right(),
                Minos::MinoS(min) => min.rotate_right(),
                Minos::MinoT(min) => min.rotate_right(),
                Minos::MinoZ(min) => min.rotate_right(),
            }
        }

        pub fn _rotate_left(&mut self) {
            match self {
                Minos::MinoI(min) => min.rotate_left(),
                Minos::MinoJ(min) => min.rotate_left(),
                Minos::MinoL(min) => min.rotate_left(),
                Minos::MinoO(min) => min.rotate_left(),
                Minos::MinoS(min) => min.rotate_left(),
                Minos::MinoT(min) => min.rotate_left(),
                Minos::MinoZ(min) => min.rotate_left(),
            }
        }

        pub fn get_shape(&self) -> [[usize; 4]; 4] {
            match self {
                Minos::MinoI(min) => min.get_shape::<I>(),
                Minos::MinoJ(min) => min.get_shape::<J>(),
                Minos::MinoL(min) => min.get_shape::<L>(),
                Minos::MinoO(min) => min.get_shape::<O>(),
                Minos::MinoS(min) => min.get_shape::<S>(),
                Minos::MinoT(min) => min.get_shape::<T>(),
                Minos::MinoZ(min) => min.get_shape::<Z>(),
            }
        }
    }

    use rand::seq::SliceRandom; // なぜかshuffleに必要
    pub fn get_mino_sets() -> Vec<Minos> {
        let mut sets = vec![
            get_default_mino("I"),
            get_default_mino("J"),
            get_default_mino("L"),
            get_default_mino("O"),
            get_default_mino("S"),
            get_default_mino("T"),
            get_default_mino("Z"),
        ];
        // vector shuffling
        let mut rng = rand::thread_rng();
        sets.shuffle(&mut rng);
        sets
    }

    pub fn get_default_mino(name: &str) -> Minos {
        match name {
            "I" => Minos::MinoI(I::default()),
            "J" => Minos::MinoJ(J::default()),
            "L" => Minos::MinoL(L::default()),
            "O" => Minos::MinoO(O::default()),
            "S" => Minos::MinoS(S::default()),
            "T" => Minos::MinoT(T::default()),
            "Z" => Minos::MinoZ(Z::default()),
            _ => panic!("illegal name for Minos at `get_default_mino`"),
        }
    }

    struct MinoHandler {
        time: f32,
    }
    pub fn update(board: &Vec<Vec<usize>>, mino: &mut Option<Minos>) {
        // println!("Here is `update`")

    }


    pub trait Mino {

        fn get_state(&self) -> &State;

        fn set_shape(&mut self, state: State);

        fn get_position(&self) -> Point;

        fn set_position(&mut self, point: Point);

        fn _shift(&mut self, x: isize, y: isize) {
            let point = self.get_position();
            self.set_position(Point {x: point.x + (x as f32), y: point.y + (y as f32)});
        }

        fn rotate_right(&mut self) {
            self.set_shape(
                match self.get_state() {
                    State::State0 => State::State1,
                    State::State1 => State::State2,
                    State::State2 => State::State3,
                    State::State3 => State::State0,
                }
            );
        }

        fn rotate_left(&mut self) {
            self.set_shape(
                match self.get_state() {
                    State::State0 => State::State3,
                    State::State1 => State::State0,
                    State::State2 => State::State1,
                    State::State3 => State::State2,
                }
                
            );
        }

        fn get_shape<T: MinoShape>(&self) -> [[usize; 4]; 4] {
            match self.get_state() {
                State::State0 => T::SHAPE0,
                State::State1 => T::SHAPE1,
                State::State2 => T::SHAPE2,
                State::State3 => T::SHAPE3,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct I {
        pub state: State,
        position: Point, //the position in tetris board
    }

    impl Mino for I {

        fn set_shape(&mut self, state: State) {
            self.state = state;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn get_position(&self) -> Point {
            self.position
        }

        fn set_position(&mut self, point: Point) {
            self.position = point;
        }

    }

    impl MinoShape for I { // 4 * 4 * 4配列
        const SHAPE0: [[usize; 4]; 4] = [[0, 0, 0, 0],
                                         [1, 1, 1, 1],
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[0, 0, 1, 0],
                                         [0, 0, 1, 0],
                                         [0, 0, 1, 0],
                                         [0, 0, 1, 0]];

        const SHAPE2: [[usize; 4]; 4] = [[0, 0, 0, 0],
                                         [0, 0, 0, 0],
                                         [1, 1, 1, 1],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[0, 1, 0, 0],
                                         [0, 1, 0, 0],
                                         [0, 1, 0, 0],
                                         [0, 1, 0, 0]];
    }

    impl Default for I {
        fn default() -> Self {
        // 4マス目が左端にくる
        Self { state: State::State0, position: Point {x: 3.0, y: -1.0}}
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct J {
        pub state: State,
        position: Point,
    }

    impl Mino for J {
        fn set_shape(&mut self, state: State) {
            self.state = state;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn get_position(&self) -> Point {
            self.position
        }

        fn set_position(&mut self, point: Point) {
            self.position = point;
        }
    }

    impl MinoShape for J {
        const SHAPE0: [[usize; 4]; 4] = [[2, 0, 0, 0],
                                         [2, 2, 2, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[0, 2, 2, 0],
                                         [0, 2, 0, 0], 
                                         [0, 2, 0, 0],
                                         [0, 0, 0, 0]];

        
        const SHAPE2: [[usize; 4]; 4] = [[0, 0, 0, 0],
                                         [2, 2, 2, 0], 
                                         [0, 0, 2, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[0, 2, 0, 0],
                                         [0, 2, 0, 0], 
                                         [2, 2, 0, 0],
                                         [0, 0, 0, 0]];
    }

    impl Default for J {
        fn default() -> Self {
        Self { state: State::State0, position: Point {x: 3.0, y: -1.0}}
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct L {
        pub state: State,
        position: Point,
    }

    impl Mino for L {
        fn set_shape(&mut self, state: State) {
            self.state = state;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn get_position(&self) -> Point {
            self.position
        }

        fn set_position(&mut self, point: Point) {
            self.position = point;
        }
    }

    impl MinoShape for L {
        const SHAPE0: [[usize; 4]; 4] = [[0, 0, 3, 0],
                                         [3, 3, 3, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[0, 3, 0, 0],
                                         [0, 3, 0, 0], 
                                         [0, 3, 3, 0],
                                         [0, 0, 0, 0]];

        const SHAPE2: [[usize; 4]; 4] = [[0, 0, 0, 0],
                                         [3, 3, 3, 0], 
                                         [3, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[3, 3, 0, 0],
                                         [0, 3, 0, 0], 
                                         [0, 3, 0, 0],
                                         [0, 0, 0, 0]];
    }

    impl Default for L {
        fn default() -> Self {
        Self { state: State::State0, position: Point {x: 3.0, y: -1.0}}
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct O {
        pub state: State,
        position: Point,
    }

    impl Mino for O {
        fn set_shape(&mut self, state: State) {
            self.state = state;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn get_position(&self) -> Point {
            self.position
        }

        fn set_position(&mut self, point: Point) {
            self.position = point;
        }
    }

    impl MinoShape for O {
        const SHAPE0: [[usize; 4]; 4] = [[0, 4, 4, 0], // when init, setting center will useful
                                         [0, 4, 4, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[0, 4, 4, 0],
                                         [0, 4, 4, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE2: [[usize; 4]; 4] = [[0, 4, 4, 0],
                                         [0, 4, 4, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[0, 4, 4, 0],
                                         [0, 4, 4, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];
    }

    impl Default for O {
        fn default() -> Self {
        Self { state: State::State0, position: Point {x: 3.0, y: -1.0}}
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct S {
        pub state: State,
        position: Point,
    }

    impl Mino for S {
        fn set_shape(&mut self, state: State) {
            self.state = state;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn get_position(&self) -> Point {
            self.position
        }

        fn set_position(&mut self, point: Point) {
            self.position = point;
        }
    }

    impl MinoShape for S {
        const SHAPE0: [[usize; 4]; 4] = [[0, 5, 5, 0],
                                         [5, 5, 0, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[0, 5, 0, 0],
                                         [0, 5, 5, 0], 
                                         [0, 0, 5, 0],
                                         [0, 0, 0, 0]];

        const SHAPE2: [[usize; 4]; 4] = [[0, 0, 0, 0],
                                         [0, 5, 5, 0], 
                                         [5, 5, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[5, 0, 0, 0],
                                         [5, 5, 0, 0], 
                                         [0, 5, 0, 0],
                                         [0, 0, 0, 0]];
    }

    impl Default for S {
        fn default() -> Self {
        Self { state: State::State0, position: Point {x: 3.0, y: -1.0}}
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct T {
        pub state: State,
        position: Point,
    }

    impl Mino for T {
        fn set_shape(&mut self, state: State) {
            self.state = state;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn get_position(&self) -> Point {
            self.position
        }

        fn set_position(&mut self, point: Point) {
            self.position = point;
        }

    }

    impl MinoShape for T {
        const SHAPE0: [[usize; 4]; 4] = [[0, 6, 0, 0],
                                         [6, 6, 6, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[0, 6, 0, 0],
                                         [0, 6, 6, 0], 
                                         [0, 6, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE2: [[usize; 4]; 4] = [[0, 0, 0, 0],
                                         [6, 6, 6, 0], 
                                         [0, 6, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[0, 6, 0, 0],
                                         [6, 6, 0, 0], 
                                         [0, 6, 0, 0],
                                         [0, 0, 0, 0]];
    }

    impl Default for T {
        fn default() -> Self {
            Self { state: State::State0, position: Point {x: 3.0, y: -1.0}}
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Z {
        pub state: State,
        position: Point,
    }

    impl Mino for Z {
        fn set_shape(&mut self, state: State) {
            self.state = state;
        }

        fn get_state(&self) -> &State {
            &self.state
        }

        fn get_position(&self) -> Point {
            self.position
        }

        fn set_position(&mut self, point: Point) {
            self.position = point;
        }
    }

    impl MinoShape for Z {
        const SHAPE0: [[usize; 4]; 4] = [[7, 7, 0, 0],
                                         [0, 7, 7, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[0, 0, 7, 0],
                                         [0, 7, 7, 0], 
                                         [0, 7, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE2: [[usize; 4]; 4] = [[0, 0, 0, 0],
                                         [7, 7, 0, 0], 
                                         [0, 7, 7, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[0, 7, 0, 0],
                                         [7, 7, 0, 0], 
                                         [7, 0, 0, 0],
                                         [0, 0, 0, 0]];
    }

    impl Default for Z {
        fn default() -> Self {
        Self { state: State::State0, position: Point {x: 3.0, y: -1.0}}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::mino::*;

    #[test]
    fn rotate() {
        let m1 = get_default_mino("L");
        assert!(match m1 {
            Minos::MinoL(_) => {
                true
            },
            _ => false,
        });

        let m1 = get_default_mino("L");
        let mut m2 = L::default();
        let mut m3 = match m1 {
            Minos::MinoL(mino) => mino,
            _ => panic!("illegal"),
        };

        for _ in 0..4 {
            m2.rotate_left();
        }
        m3.rotate_left();
        m3.rotate_right();
        assert_eq!(m2, m3);
    }

    #[test]
    fn shift() {
        let mut m1 = S::default();
        let mut m2 = get_default_mino("S");

        m1._shift(1, 3);
        m1._shift(-2, 4);

        m2.shift(-1, 7);

        assert_eq!(Minos::MinoS(m1), m2);
    }

    #[test]
    fn settable() {
        let empty = empty_board();
        let mut m1 = get_default_mino("O");
        m1.shift(4, 0);
        assert!(m1.is_settable(&empty));

        let mut m1 = get_default_mino("I");
        m1._rotate_right();
        m1.shift(0, 18);
        let mut empty = empty_board();
        // m1.place(&mut empty);
        debug(&mut m1, &empty);
        let mut empty = empty_board();
        assert!(!m1.is_settable(&empty));
        let empty = empty_board();

        let mut board = empty_board();
        board[3][0 + 5] = 5;
        let mut m1 = get_default_mino("I");
        assert!(!m1.is_settable(&board));

        m1._rotate_right();
        assert!(m1.is_settable(&board));

        //x: -1のときのtest
        let mut m1 = get_default_mino("S");
        m1._rotate_right();
        m1.shift(-4, 7);
        assert!(m1.is_settable(&empty));

        let mut m1 = get_default_mino("I");
        // m1._rotate_right();
        m1.set_position(3, 18);
        debug(&mut m1, &empty);
        assert!(m1.is_settable(&empty));

        let mut m1 = get_default_mino("O");
        m1.set_position(3, -1);
        assert!(m1.is_settable(&empty));
    }

    fn debug(m1: &mut Minos, board: &Vec<Vec<usize>>) {
        //For Debug
        for row in &board[..] {
            println!("{:?}", row);
        }
        println!("=====");
        let shape = m1.get_shape();
        for i in 0..4 {
            for j in 0..4 {
                print!("{}", shape[j][i]);
            }
            println!("");
        }
        println!("=====");
        println!("{:?}", m1.get_position());
    }

    fn empty_board() -> Vec<Vec<usize>> {
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
        colors
    }
}