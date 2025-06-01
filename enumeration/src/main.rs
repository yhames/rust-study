#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

enum Message {
    StartGame,
    WinPoint { who: String },
    ChaangePlayerName(String),
}

fn main() {
    let color = Color::Red;

    match color {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }

    let m1 = Message::StartGame;
    let m2 = Message::WinPoint { who: String::from("Alice") };
    let m3 = Message::ChaangePlayerName(String::from("Bob"));

    match m1 {
        Message::StartGame => println!("Game started!"),
        Message::WinPoint { who } => println!("{} wins a point!", who),
        Message::ChaangePlayerName(_) => println!("Player name changed"),
        _ => println!("Unknown message"),   // This line is not necessary, but shows how to handle other cases
    }

    let some_number  = Option::Some(5);
    match some_number {
        Option::Some(n) => println!("We have a number: {}", n),
        Option::None => println!("No number found"),
    }

    let no_number: Option<i32> = Option::None;
    match no_number {
        Option::Some(n) => println!("We have a number: {}", n),
        Option::None => println!("No number found"),
    }
}
