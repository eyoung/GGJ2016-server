pub struct Scene {
    arms: isize,
    legs: isize,
    head: isize,
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            arms: 0,
            legs: 0,
            head: 0
        }
    }
}