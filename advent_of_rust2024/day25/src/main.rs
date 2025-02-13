fn main() {
    println!("Hello, world!");
    let last_message = send_message_to_santa();

    println!("The last message = {}", last_message);
}
pub fn send_message_to_santa() -> String {
    String::from("I've been nice")
}
