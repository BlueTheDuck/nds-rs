use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
pub struct CbEnumRenamer<'s> {
    replacements: &'s [[&'static str; 2]],
}
impl<'s> CbEnumRenamer<'s> {
    pub fn new(replacements: &'s [[&'static str; 2]]) -> Self {
        Self { replacements }
    }

    pub fn new_boxed(replacements: &'s [[&'static str; 2]]) -> Box<Self> {
        Box::new(Self { replacements })
    }
}
impl<'m> ParseCallbacks for CbEnumRenamer<'m> {
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        _original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        self.replacements.iter().find_map(|[o, r]| {
            if *o == _original_variant_name {
                Some(r.to_string())
            } else {
                None
            }
        })
    }
}
