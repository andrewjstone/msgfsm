#![feature(specialization)]

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tests {

    use std::convert::{From, Into};

    trait Transition<Input> {
        fn next(self, msg: Input, output: &mut Vec<OutputMsg>) -> States;
    }

    macro_rules! next {
        ($state:ident, $msg:ident, $output:ident) => {
            match $msg {
                InputMsg::Meow(meow) => $state.next(meow, $output),
                InputMsg::Eat(eat) => $state.next(eat, $output)
            }
        }
    }

    #[derive(Debug)]
    enum States {
        Full(Full),
        Empty(Empty)
    }

    impl States {
        fn next(self, msg: InputMsg, output: &mut Vec<OutputMsg>) -> States {
            match self {
                States::Full(full) => next!(full, msg, output),
                States::Empty(empty) => next!(empty, msg, output)
            }
        }
    }

    // States
    #[derive(Debug)]
    struct Full {contents: u8}
    #[derive(Debug)]
    struct Empty;

    impl From<Full> for States {
        fn from(full: Full) -> States {
            States::Full(full)
        }
    }

    impl From<Empty> for States {
        fn from(empty: Empty) -> States {
            States::Empty(empty)
        }
    }

    // Messages
    struct Meow;
    struct Eat {amount: u8}

    enum InputMsg {
        Meow(Meow),
        Eat(Eat)
    }

    #[derive(Debug)]
    enum OutputMsg {
        Poop,
        Nap
    }

    // State transition for Full state where Eat is received
    impl Transition<Eat> for Full {
        fn next(mut self, eat: Eat, output: &mut Vec<OutputMsg>) -> States {
            // If the cat finishes their food, they poop and nap
            if eat.amount >= self.contents {
                output.push(OutputMsg::Poop);
                output.push(OutputMsg::Nap);
                return Empty.into();
            }
            // If the cat only eats some of the food, they just nap
            output.push(OutputMsg::Nap);
            self.contents -= eat.amount;
            self.into()
        }
    }

    // State transition for Empty state where Meow is received
    impl Transition<Meow> for Empty {
        fn next(self, _: Meow, _: &mut Vec<OutputMsg>) -> States {
            Full {contents: 100}.into()
        }
    }

    // Any transition where Meow is received but shouldn't be handled in State T
    // Specifically, the Full state doesn't handle Meow messages
    impl<T> Transition<Meow> for T
        where States: From<T>
    {
       default fn next(self, _: Meow, _: &mut Vec<OutputMsg>) -> States {
            self.into()
        }
    }

    // Any transition where Eat is received but shouldn't be handled in State T
    // Specifically the Empty state doesn't handle Eat messages
    impl<T> Transition<Eat> for T
        where States: From<T>
    {
        default fn next(self, _: Eat, _: &mut Vec<OutputMsg>) -> States {
            self.into()
        }
    }


    #[test]
    fn basic() {
        let mut output = Vec::new();
        let state = States::Full(Full {contents: 100});
        let state = state.next(InputMsg::Eat(Eat {amount: 70}), &mut output);
        assert_matches!(state, States::Full(_));
        assert_eq!(output.len(), 1);
        assert_matches!(output.pop(), Some(OutputMsg::Nap));

        let state = state.next(InputMsg::Eat(Eat {amount: 30}), &mut output);
        assert_matches!(state, States::Empty(_));
        assert_eq!(output.len(), 2);
        assert_matches!(output.pop(), Some(OutputMsg::Nap));
        assert_matches!(output.pop(), Some(OutputMsg::Poop));

        let state = state.next(InputMsg::Meow(Meow), &mut output);
        assert_matches!(state, States::Full(_));
    }
}
