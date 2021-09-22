use practice::greetings;
use practice::people;

fn main() {
    greetings::greet(people::People::Tyler);
    greetings::greet(people::People::Alec);
    greetings::greet(people::People::Andrew);
}
