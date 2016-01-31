pub struct Scene {
    pub arms: isize,
    pub legs: isize,
    pub head: isize,
    pub body: isize,
    pub scene_number: isize
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            arms: 0,
            legs: 0,
            head: 0,
            body: 0,
            scene_number: 0
        }
    }

    pub fn next(&mut self) {
        self.arms = 0;
        self.legs = 0;
        self.head = 0;
        self.body = 0;
        self.scene_number += 1;
    }
}