#[derive(Debug, Clone)]
pub struct Spec {
    pub name: proc_macro2::Ident,
    pub fields: Vec<SpecField>,
}

#[derive(Debug, Clone)]
pub struct SpecField {
    pub name: proc_macro2::Ident,
    pub start: usize,
    pub size: usize,
}
