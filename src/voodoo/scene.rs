pub struct Scene {
    pub arm_left: isize,
    pub arm_right: isize,
    pub leg_left: isize,
    pub leg_right: isize,
    pub head: isize,
    pub body: isize,
    pub scene_number: isize
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            arm_left: 0,
            arm_right: 0,
            leg_left: 0,
            leg_right: 0,
            head: 0,
            body: 0,
            scene_number: 0
        }
    }

    pub fn next(&mut self) {
        self.arm_left = 0;
        self.arm_right = 0;
        self.leg_left = 0;
        self.leg_right = 0;
        self.head = 0;
        self.body = 0;
        self.scene_number += 1;
    }
}