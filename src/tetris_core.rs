pub mod mino {
    use iced::Point;
    use std::default::Default;

    #[derive(Debug)]
    pub enum State {
        State0,
        State1,
        State2,
        State3,
    }

    pub trait Mino {

        const SHAPE0: [[usize; 4]; 4];
        const SHAPE1: [[usize; 4]; 4];
        const SHAPE2: [[usize; 4]; 4];
        const SHAPE3: [[usize; 4]; 4];

        fn get_state(&self) -> &State;

        fn set_shape(&mut self, state: State);

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

        fn get_shape(&self) -> [[usize; 4]; 4] {
            match self.get_state() {
                State::State0 => Self::SHAPE0,
                State::State1 => Self::SHAPE1,
                State::State2 => Self::SHAPE2,
                State::State3 => Self::SHAPE3,
            }
        }
    }

    #[derive(Debug)]
    struct I {
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
        Self { state: State::State0, position: Point {x: 0.0, y: 0.0}}
    }
    }

    struct J {
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

    struct L {
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
                                         [0, 0, 3, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[0, 3, 0, 0],
                                         [0, 3, 0, 0], 
                                         [3, 3, 0, 0],
                                         [0, 0, 0, 0]];
    }

    struct O {
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

        const SHAPE0: [[usize; 4]; 4] = [[4, 4, 0, 0],
                                         [4, 4, 0, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE1: [[usize; 4]; 4] = [[4, 4, 0, 0],
                                         [4, 4, 0, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE2: [[usize; 4]; 4] = [[4, 4, 0, 0],
                                         [4, 4, 0, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];

        const SHAPE3: [[usize; 4]; 4] = [[4, 4, 0, 0],
                                         [4, 4, 0, 0], 
                                         [0, 0, 0, 0],
                                         [0, 0, 0, 0]];
    }

    struct S {
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

    struct T {
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

    struct Z {
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
}