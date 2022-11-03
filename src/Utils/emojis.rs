pub struct Emoji {
    unicode: String,
    description: String,
    hp: i32,
    name: String,
}

impl Emoji {
    fn new(unicode: String, description: String, hp: i32, name: String) -> Self {
        Self {
            unicode,
            description,
            hp,
            name,
        }
    }
}

pub enum Emojis {
    AngryDevil(Emoji::new()),
}
