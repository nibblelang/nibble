mod label;

use label::LabelGenerator;

#[derive(Debug)]
pub struct Context {
    labels: LabelGenerator,
}
impl Context {
    pub fn empty() -> Self {
        Self {
            labels: LabelGenerator::new(),
        }
    }
    pub fn unique_label(&mut self) -> String {
        self.labels.unique_label()
    }
}
