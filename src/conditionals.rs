pub fn run() {
    let age = 18;
    let check_id = false;
    let knows_person_of_age = true;

    // if else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: what would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: sorry you have to leave.");
    } else {
        println!("Bartender: I need to see ur ID");
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age {}", is_of_age);
}
