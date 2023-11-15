pub struct AccelerometerData {
    x: f32,
    y: f32,
    z: f32,
}

impl AccelerometerData {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn get_theta(&self) -> f32 {
        todo!()
    }

    pub fn get_phi(&self) -> f32 {
        todo!()
    }
}
