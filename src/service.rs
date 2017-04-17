use server::Message;

pub trait Service {
    fn ready(&mut self, message: Message) -> Message;
}
