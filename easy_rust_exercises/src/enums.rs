// Enums are similar to structs, but, structs are for many things together, while enums are for many choices together.
enum PeopleInAClassroom {
    Teachers(String),
    Cleaners(String),
}

enum DayInWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}


pub fn enums_test() {
    use DayInWeek::*;
    
    let class_state_monday = create_classroom_state(Monday);
    let class_state_tuesday = create_classroom_state(Tuesday);
    let class_state_wednesday = create_classroom_state(Wednesday);
    let class_state_thursday = create_classroom_state(Thursday);
    let class_state_friday = create_classroom_state(Friday);
    let class_state_saturday = create_classroom_state(Saturday);
    let class_state_sunday = create_classroom_state(Sunday);
    
    read_classroom_state(&class_state_monday);
    read_classroom_state(&class_state_tuesday);
    read_classroom_state(&class_state_wednesday);
    read_classroom_state(&class_state_thursday);
    read_classroom_state(&class_state_friday);
    read_classroom_state(&class_state_saturday);
    read_classroom_state(&class_state_sunday);
}

fn create_classroom_state(day: DayInWeek) -> PeopleInAClassroom {
    match day {
        DayInWeek::Monday => PeopleInAClassroom::Teachers("maths".to_string()),
        DayInWeek::Tuesday => PeopleInAClassroom::Teachers("english".to_string()),
        DayInWeek::Wednesday => PeopleInAClassroom::Teachers("programing".to_string()),
        DayInWeek::Thursday => PeopleInAClassroom::Teachers("geography".to_string()),
        DayInWeek::Friday => PeopleInAClassroom::Teachers("spleling".to_string()),
        DayInWeek::Saturday => PeopleInAClassroom::Cleaners("furniture".to_string()),
        DayInWeek::Sunday => PeopleInAClassroom::Cleaners("floors".to_string()),
    }
}

fn read_classroom_state(state: &PeopleInAClassroom) {
    match state {
        PeopleInAClassroom::Teachers(subject) => println!("There are teachers teaching {} now.", subject),
        PeopleInAClassroom::Cleaners(item) => println!("There are cleaners cleaning {} now.", item),
    };
}
