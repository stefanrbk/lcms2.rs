use crate::Context;

pub trait IoHandler {
    fn context_id(&self) -> &Context;
    fn used_space(&self) -> usize;
    fn reported_size(&self) -> usize;
    fn physical_file(&self) -> &String;
    fn read(&mut self, buffer: &mut [u8], size: usize, count: usize) -> usize;
    fn seek(&mut self, offset: usize) -> bool;
    fn close(&mut self) -> bool;
    fn tell(&mut self) -> usize;
    fn write(&mut self, size: usize, buffer: &[u8]) -> bool;
}
