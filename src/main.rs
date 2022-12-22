#[derive(Debug)]
pub struct Neuron {
    pub num: f64,
}
impl Neuron {
    pub fn forward(&self) -> f32 {
        let mut output = 0.0;

        output
    }
}
#[derive(Debug)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}
impl Layer {
    pub fn forward(&self) -> Vec<f32> {
        let mut output = Vec::new();
        for neuron in &self.neurons {
            output.push(neuron.forward());
        }
        output
    }
}
#[derive(Debug)]
pub struct Network {
    pub layers: Vec<Layer>,
    pub forward: fn(Vec<f32>) -> Vec<f32>,
}

fn main() {}
