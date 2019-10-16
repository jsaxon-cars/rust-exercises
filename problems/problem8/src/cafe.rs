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

    /// Open (and close) the doors, letting "in" a set of visitors.  Let the
    /// visitors use a computer till they're done, then let the next visitor
    /// do the same till all visitors have finished.  Then close the doors.
    /// * Create message channel
    /// * If there is a free computer allocate it to the next visitor
    /// * Wait for any computers to become free and summarize that visit.
    /// * Continue until all visitors have finished.
    /// * Close the doors.
    /// 
    /// Note: Could extend to add and remove computers over time and have 
    /// random visitors come in too.  Use a command line interface.
    pub fn open_doors(mut self, mut visitors: Vec<Visitor>) {
        use std::time::Duration;

        println!("We're open!");

        println!("{} visitors come in...", visitors.len());

        let (tx, rx) = mpsc::channel();
        while let Some(next_up) = visitors.pop() {
            println!("Next up: Visitor {}.", next_up.name());
            // So, we're waiting a second here.  Not the best, we could 
            // do it with the message itself perhaps?  Next attempt.
            while !self.open_computers() {
                thread::sleep(Duration::from_secs(1));
                if let Result::Ok(msg) = rx.try_recv() {
                    self.handle_msg(msg);
                }
            }
            println!("A computer is ready!");
            let tx = mpsc::Sender::clone(&tx);
            self.allocate_computer(next_up, tx);
        };
        drop(tx);
        while let Result::Ok(msg) = rx.recv() {
            self.handle_msg(msg)
        }

        println!("No more visitors.  Closing the doors!");
    }

    fn open_computers(&self) -> bool {
        self.available_computers > 0
    }

    /// Go through these steps:
    /// * Announce a visitor
    /// * Allocate a computer (subtract 1)
    /// * Let them visit for however long they want, and then
    /// * Sending a summary of their visit to our channel to indicate when they're
    /// *   all done. 
    fn allocate_computer(&mut self, v: Visitor, sender: mpsc::Sender<String>) {
        self.available_computers = self.available_computers - 1;
        thread::spawn(move || {
            println!("{}", v.visit_start());
            v.visit();
            // If there's some failure here, panic!  It won't happen since our
            // receiver isn't going anywhere in this situation.
            sender.send(v.visit_summary()).unwrap();
        });
    }

    /// Print the string message (assuming it's a thread completion message of 
    /// the visit) and make a computer available again (Add 1).
    fn handle_msg(&mut self, msg: String) {
        println!("{}", msg);
        self.available_computers = self.available_computers + 1;
    }
}
