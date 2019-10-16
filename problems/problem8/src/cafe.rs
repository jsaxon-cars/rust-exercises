use crate::visitor::Visitor;
use std::sync::mpsc;
use std::thread;
use std::vec::Vec;

/// Our little internet cafe, which is a group of visitors and some number of
/// computers. For this exercise there's no need to allow visitors to join
/// throughout the day (or storm out angrily after a long wait). We'll focus
/// only on serving the group we start with.
#[derive(Debug)]
pub struct Cafe {
    // visitors: Vec<Visitor>,
    available_computers: u32,
}

impl Cafe {
    /// Creates a brand new internet cafe in no time at all. Amazing!
    pub fn new(available_computers: u32) -> Self {
        println!(
            "Setting up the cafe with {} computers!",
            available_computers
        );

        let cafe = Cafe {
            // visitors: vec![],
            available_computers: available_computers,
        };

        println!("The cafe is ready!");

        cafe
    }

    /// This function should create our message channel, decide when there
    /// is a visitor and a free computer, then make use of
    /// `self.allocate_computer` (check its type signature for hints).
    /// It will also need to use `self.handle_msg` in two places (check its
    /// comments for another hint).
    pub fn open_doors(mut self, mut visitors: Vec<Visitor>) {
        use std::time::Duration;

        println!("We're open!");

        println!("{} visitors come in...", visitors.len());

        let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

        while let Some(next_up) = visitors.pop() {
            println!("Next up: Visitor {}.", next_up.name());
            while !self.open_computers() {
                thread::sleep(Duration::from_secs(1));
                if let Result::Ok(msg) = rx.try_recv() {
                    self.handle_msg(msg);
                }
            }
            println!("A computer is ready!");
            let tx = mpsc::Sender::clone(&tx);
            self.allocate_computer(next_up, tx);
        }
        for received in rx {
            println!("{}", received);
        }

        println!("No more visitors!");

        println!("I guess we're closing the doors.");
    }

    fn open_computers(&self) -> bool {
        self.available_computers > 0
    }

    /// Here we need to go through all the steps of announcing a visitor, giving
    /// them a computer, letting them visit for however long they want, and then
    /// sending a summary of their visit to our channel to indicate when they're
    /// all done. Check `visitor.rs` to see what methods you have available to
    /// you.
    fn allocate_computer(&mut self, v: Visitor, sender: mpsc::Sender<String>) {
        self.available_computers = self.available_computers - 1;
        thread::spawn(move || {
            println!("{}", v.visit_start());
            v.visit();
            sender.send(v.visit_summary()).unwrap();
        });
    }

    /// We have to be prepared to receive messages at different times (while
    /// some visitors are still waiting, and after all computers are allocated
    /// but might still be in use). This helper function will print the summary
    /// and make a computer available again.
    fn handle_msg(&mut self, msg: String) {
        println!("{}", msg);
        self.available_computers = self.available_computers + 1;
    }
}
