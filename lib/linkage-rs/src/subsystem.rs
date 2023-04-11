pub trait Subsystem {
    fn setup(&mut self);
    fn tick(&mut self);
    fn shutdown(&mut self);
}
