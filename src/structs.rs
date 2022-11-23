// Structs - used to create custom data types

/// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/// Tupple Struct
struct TColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct a person
    fn new(first: &str, last: &str) -> Self {
        Self {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TColor(255, 0, 0);
    tc.2 = 100;
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

    // Person
    let mut p = Person::new("John", "Doe");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    p.set_last_name("Mary");
    println!("{}", p.full_name());
    println!("{:#?}", p.to_tuple());
}
