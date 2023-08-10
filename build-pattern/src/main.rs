// https://youtu.be/5DWU-56mjmg

struct Student {
    name: String,
    age: u8,
    phone: String,
    email: String,
    address: String,
}

struct StudentBuilder {
    name: String,
    age: u8,
    phone: Option<String>,
    email: Option<String>,
    address: Option<String>,
}

impl StudentBuilder {
    fn new(
        name: String,
        age: u8,
        phone: Option<String>,
        email: Option<String>,
        address: Option<String>,
    ) -> Self {
        Self {
            name,
            age,
            phone,
            email,
            address,
        }
    }

    fn phone(&mut self, phone: String) -> &mut Self {
        self.phone = Some(phone);
        self
    }

    fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    fn address(&mut self, address: String) -> &mut Self {
        self.address = Some(address);
        self
    }

    fn build(&mut self) -> Student {
        Student {
            name: self.name.clone(),
            age: self.age,
            phone: self.phone.clone().unwrap_or_default(),
            email: self.email.clone().unwrap_or_default(),
            address: self.address.clone().unwrap_or_default(),
        }
    }
}

impl Student {
    fn new(name: String, age: u8) -> StudentBuilder {
        StudentBuilder::new(name, age, None, None, None)
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let phone = if self.phone.is_empty() {
            String::new()
        } else {
            format!(" | {}", self.phone)
        };
        let email = if self.email.is_empty() {
            String::new()
        } else {
            format!(" | {}", self.email)
        };
        let address = if self.address.is_empty() {
            String::new()
        } else {
            format!(" | {}", self.address)
        };
        write!(
            f,
            "{} | {}{}{}{}",
            self.name, self.age, phone, email, address
        )
    }
}

fn main() {
    let _student_a = Student::new("John".to_string(), 20).build();
    let _student_b = Student::new("Jane".to_string(), 21)
        .phone("1234567890".to_string())
        .build();
    let _student_c = Student::new("Joe".to_string(), 22)
        .phone("1234567890".to_string())
        .email("hello@world".to_string())
        .build();
    let _student_d = Student::new("Jill".to_string(), 23)
        .phone("1234567890".to_string())
        .email("hello@world".to_string())
        .address("1234 Main St".to_string())
        .build();
    println!("{}", _student_a);
    println!("{}", _student_b);
    println!("{}", _student_c);
    println!("{}", _student_d);
}
