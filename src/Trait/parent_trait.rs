trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_languages(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {}, My university is {} , My favorite language is {},My git username is {}",
        student.name(),
        student.university(),
        student.fav_languages(),
        student.git_username()
    )
}

#[derive(Debug)]
struct StructStudent {
    name: String,
    university: String,
    fav_languages: String,
    git_username: String,
}

impl CompSciStudent for StructStudent {
    fn git_username(&self) -> String {
        self.git_username.to_string()
    }
}
impl Student for StructStudent {
    fn university(&self) -> String {
        self.university.to_string()
    }
}
impl Person for StructStudent {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl Programmer for StructStudent {
    fn fav_languages(&self) -> String {
        self.fav_languages.to_string()
    }
}

fn main() {
    let student = StructStudent {
        name: "小明".into(),
        university: "科技大学".into(),
        fav_languages: "Rust".into(),
        git_username: "katte".into(),
    };
    let result = comp_sci_student_greeting(&student);

    println!("{}", result);

    println!("{:#?}", student);
}
