use problem8::cafe::Cafe;
use problem8::visitor::Visitor;

/// Runs our internet cafe simulation, announcing when visitors go online and
/// reporting the length of their visit afterwards. When one visitor sends a
/// message that they're done with the computer, the next available visitor
/// will jump online.
fn main() {
    println!("Running the Internet Cafe!");

    // start up the cafe with our visitor group and 10 available computers
    let cafe = Cafe::new(10);

    // create a vector of visitors
    let visitors = Visitor::generate_visitors(20);

    cafe.open_doors(visitors);
}