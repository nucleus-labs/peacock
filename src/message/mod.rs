
/// A generic message containing the ID of the widget that produced the message
/// and the message itself
#[derive(Debug, Clone)]
pub struct MessageGeneric(pub String, pub MessageGenericInner);

/// An enum containing a value for each widget that may produce a message
#[derive(Debug, Clone)]
pub enum MessageGenericInner {
    Button,
    Checkbox(bool),
}

/// Function for receiving messages produced by widgets.
/// 
/// These will be registered with the ApplicationContext and be called
/// during the `update` function.
pub type MessageReceiver<State> = dyn Fn(&mut super::ApplicationContext<State>, MessageGeneric) -> ();
