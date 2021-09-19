pub mod mino {
    enum State {
        State0,
        State1,
        State2,
        State3,
    }

    fn next_state(state: &State) -> State  {
        match state {
            State::State0 => State::State1,
            State::State1 => State::State2,
            State::State2 => State::State3,
            State::State3 => State::State0,
        }
    }

    fn ex_state(state: &State) -> State {
        match state {
            State::State0 => State::State3,
            State::State1 => State::State0,
            State::State2 => State::State1,
            State::State3 => State::State2,
        }
    }

    pub trait Mino {

        /*
        const SHAPE0: [[usize; 4]; 4];
        const SHAPE1: [[usize; 4]; 4];
        const SHAPE2: [[usize; 4]; 4];
        const SHAPE3: [[usize; 4]; 4];
        */

        fn rotate_right(&mut self) {
        }

        fn rotate_left(&mut self);

        fn get_shape() -> Vec<usize>;
    }

    struct I {
        pub state: State,
        shape: Vec<usize>,
    }

    impl Mino for I {


        fn rotate_right(&mut self) {
            self.state = next_state(&self.state);
        }

        fn rotate_left(&mut self) {
            self.state = ex_state(&self.state);
        }

        fn get_shape() -> Vec<usize> {
            vec![1]
        }

        /*
        const SHAPE0: [[usize; 4]; 4];

        const SHAPE1: [[usize; 4]; 4];

        const SHAPE2: [[usize; 4]; 4];

        const SHAPE3: [[usize; 4]; 4];
        */
    }

    impl I {
    }
}