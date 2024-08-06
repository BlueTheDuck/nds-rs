pub struct Layer0(());
impl LayerMarker for Layer0 {
    const LAYER_INDEX: usize = 0;
}

pub struct Layer1(());
impl LayerMarker for Layer1 {
    const LAYER_INDEX: usize = 1;
}

pub struct Layer2(());
impl LayerMarker for Layer2 {
    const LAYER_INDEX: usize = 2;
}

pub struct Layer3(());
impl LayerMarker for Layer3 {
    const LAYER_INDEX: usize = 3;
}

pub trait LayerMarker: crate::private::Sealed {
    const LAYER_INDEX: usize;
}
