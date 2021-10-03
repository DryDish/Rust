use logger::{SIZE, init_logger, error, info, warn};


#[derive(Debug)]
struct ObjectThing {
    name: String,
    age: u8
}

fn main() {
    init_logger();
    let thing = ObjectThing {
        name: "Peter".to_string(),
        age: 18
    };

    info!("Hello there");
    info!("Hello", "there!");
    warn!("Warning you about stuff");
    warn!("Warning you about this:", thing);
    error!("Big problems my mans!");
    error!("Here is your problem :", thing);

}
