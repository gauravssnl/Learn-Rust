// Interior Mutability: A Mutable Borrow to an Immutable Value

pub trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<T> {
        // return type Self also works here
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64; // note both should have same data type
        if percentage_of_max > 1.0 {
            self.messenger.send("Error: You're over your quota!");
        } else if percentage_of_max > 0.9 {
            self.messenger
                .send("Urgent warning : You've used up over 90% of your quota!");
        } else if percentage_of_max > 0.75 {
            self.messenger
                .send("Warning : You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // create a mock strcut (like a mock object)
    struct MockMessenger {
        // we have to use RefCell<T> as send() of  accpets Messenger &self not &mut self
        // it it was &mut self, we could've used sent_messages: Vec<String> here
        // RefCell<T> won't work in multi-threaded context
        sent_messages: RefCell<Vec<String>>, // store all sent messages by LimitTracker
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(Vec::new()), // or we can use vec![]
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg)); // borrow mutably
        }
    }

    #[test]
    fn it_sends_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); //  borrow immutably
    }
}
