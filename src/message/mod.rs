
/// A generic message containing the ID of the widget that produced the message
/// and the message itself
pub type MessageGeneric = (String, MessageGenericInner);

/// An enum containing a value for each widget that may produce a message
#[derive(Debug, Clone)]
pub enum MessageGenericInner {
    Button,

}

pub type MessageReceiver<State> = dyn Fn(&super::ApplicationContext<State>, MessageGeneric) -> ();
