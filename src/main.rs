#[derive(Debug)]
pub struct Neuron {
    pub num: f32,
}
impl Neuron {
    pub fn forward(&self, input: Vec<f32>) -> f32 {
        0.0
    }
}
#[derive(Debug)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}
impl Layer {
    pub fn forward(&self, input: Vec<f32>) -> Vec<f32> {
        let mut output = Vec::new();
        for neuron in &self.neurons {
            output.push(neuron.forward(input.clone()));
        }
        output
    }
}
#[derive(Debug)]
pub struct Network {
    pub layers: Vec<Layer>,
}
impl Network {
    pub fn forward(&self, input: Vec<f32>) -> Vec<f32> {
        let mut output = input;
        for layer in &self.layers {
            output = layer.forward(output);
        }
        output
    }
}
fn main() {}
