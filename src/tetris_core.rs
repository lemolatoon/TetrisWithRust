pub mod mino {
    use iced::Point;
    use std::default::Default;

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

    #[derive(Debug, Clone)]
    pub enum Minos {
        MinoI(I),
        MinoJ(J),
        MinoL(L),
        MinoO(O),
        MinoS(S),
        MinoT(T),
        MinoZ(Z),
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

    pub fn update(board: &Vec<Vec<usize>>) {

    }

    pub trait Mino {


        fn get_state(&self) -> &State;

        fn set_shape(&mut self, state: State);

        fn get_position(&self) -> Point;

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

    }

    impl MinoShape for I {
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
        Self { state: State::State0, position: Point {x: 3.0, y: 0.0}}
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
        Self { state: State::State0, position: Point {x: 3.0, y: 0.0}}
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
        Self { state: State::State0, position: Point {x: 3.0, y: 0.0}}
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
        Self { state: State::State0, position: Point {x: 3.0, y: 0.0}}
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
        Self { state: State::State0, position: Point {x: 3.0, y: 0.0}}
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
            Self { state: State::State0, position: Point {x: 3.0, y: 0.0}}
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
    }

    impl MinoShape for Z {
        const SHAPE0: [[usize; 4]; 4] = [[7, 7, 0, 0],
                                         [0, 7, 7, 0], 
                                         [0, 6, 0, 0],
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
        Self { state: State::State0, position: Point {x: 3.0, y: 0.0}}
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
            Minos::MinoL(mino) => {
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
}