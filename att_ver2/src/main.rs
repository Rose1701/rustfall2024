#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        // Implementing printing info about Student using match statement
        match &self.grade {
            GradeLevel::Bachelor => println!("Hello, my name is {}. I'm a Bachelor student majoring in {:?}.", self.name, self.major),
            GradeLevel::Master => println!("Hello, my name is {}. I'm a Master student majoring in {:?}.", self.name, self.major),
            GradeLevel::PhD => println!("Hello, my name is {}. I'm a PhD student majoring in {:?}.", self.name, self.major),
        }
    }
}

fn main() {
    let s1 = Student::new("John".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
s1.introduce_yourself();

    let s2 = Student::new("Rose".to_string(),
    GradeLevel::Master,
    Major::ComputerScience);
s2.introduce_yourself();

    let s3 = Student::new("Rick".to_string(),
    GradeLevel::PhD,
    Major::ElectricalEngineering);
s3.introduce_yourself();

}
