use candle_core::{DType, Device, Module, Result, Tensor};
use candle_lora::{LinearLayerLike, LoraConfig, LoraLinearConfig};
use candle_lora_macro::{replace_layer_fields, AutoLoraConvert};
use candle_nn::{init, Linear, VarMap};

#[replace_layer_fields]
#[derive(AutoLoraConvert, Debug)]
struct Model {
    layer: Linear,
}

impl Module for Model {
    fn forward(&self, input: &Tensor) -> Result<Tensor> {
        self.layer.forward(input)
    }
}

fn main() {
    let device = Device::Cpu;
    let dtype = DType::F32;

    let map = VarMap::new();
    let layer_weight = map
        .get(
            (10, 10),
            "layer.weight",
            init::DEFAULT_KAIMING_NORMAL,
            dtype,
            &device,
        )
        .unwrap();

    let mut model = Model {
        layer: Box::new(Linear::new(layer_weight.clone(), None)),
    };

    let loraconfig = LoraConfig::new(1, 1., None, &device, dtype);
    model.get_lora_model(
        loraconfig,
        Some(LoraLinearConfig::new(10, 10)),
        None,
        None,
        None,
    );

    let dummy_image = Tensor::zeros((10, 10), DType::F32, &device).unwrap();

    let digit = model.forward(&dummy_image).unwrap();
    println!("Output: {digit:?}");
}