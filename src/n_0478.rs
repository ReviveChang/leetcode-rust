use rand::prelude::*;

struct Solution {
    radius:f64,
    x_center:f64,
    y_center:f64,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution{radius,x_center,y_center}
    }
    
    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let r:f64 = rng.gen_range(0.0..self.radius.powf(2.0)).sqrt();
        let t:f64 = rng.gen_range(0.0..std::f64::consts::PI*2.0);
        vec![r*t.cos()+self.x_center,r*t.sin()+self.y_center]
    }
}
