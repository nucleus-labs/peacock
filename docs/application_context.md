
# Application Context

Peacock's <ins>***entire***</ins> job is managing an [`iced`] application for you. This includes
the initial construction of the application from XML, to the styling of elements using CSS,
to dynamically altering the content and behaviour during the application's runtime.

To do this, a context that holds all of the information about the application is needed, including
the user-defined `State`. The message callbacks, every single widget, source information, template
registry, etc are all a part of the application context, and as such are managed for you by peacock.

The only times you should ever interact with the application context are:
- initial construction
- telling it what to use to build the app (templates/xml)
- telling it what to use to style the app (css)
- registering callbacks so you can respond to user-interactions, etc
- dynamically updating the application's content

You should <ins>***never***</ins> be interacting with the [`iced`] foundation, as it is peacock's
job to do so on your behalf. If you find that you ***are*** interacting with it, ***please***
<ins>[open an issue](https://github.com/nucleus-labs/peacock/issues/new)</ins>.

## Minimal Usage

This example shows how to create a stateless application. It will do nothing, respond to nothing,
it is good only for displaying static pages.

```rust
use peacock::ApplicationContext;
use minijinja::context;

fn main() -> peacock::Result {
    let mut app: ApplicationContext<()> = ApplicationContext::new("Minimal Peacock Usage");

    app.read_xml_templates_auto()?;
    app.render_template_to_registry("index".into(), "index".into(), context!{})?;

    app.run()
}
```

## Basic Usage

If the user-defined state implements Default, the application context will automatically use
that if using ::new(). Otherwise, you will need to use ::new_with_state() and provide the
initial state. This is also convenient if your initial state differs from its implemented
default. Peacock does not modify this state, ever. All responsibility for doing so it on
the user, though many utilities are provided for doing so.

```rust
use peacock::ApplicationContext;
use minijinja::context;

struct MyState {
    foo: usize,
    bar: i32,
}

fn main() -> peacock::Result {
    let initial_state = MyState{
        foo: 0,
        bar: 1,
    };

    let mut app: ApplicationContext<MyState> = ApplicationContext::new_with_state(
        "Basic Peacock Usage",
        initial_state
    );

    app.read_xml_templates_auto()?;
    app.render_template_to_registry("index".into(), "index".into(), context!{})?;

    // lets assume that our 'index.xml' file contains a button element with id 'add_button'!
    app.register_message_receiver("add_button", Box::new(|ctx, msg| {
        let state_handle = ctx.get_state();
        let mut state_guard = state_handle.write().unwrap();

        state_guard.foo += 1;
        state_guard.bar -= 3;

        // update content using the updated state...
    }))

    app.run()
}
```

or, from a different approach:

```rust
use peacock::{ApplicationContext, message::MessageGeneric};
use minijinja::context;

struct MyState {
    foo: usize,
    bar: i32,
}

fn adder(ctx: &mut ApplicationContext<MyState>, msg: MessageGeneric) {
    let state_handle = ctx.get_state();
    let mut state_guard = state_handle.write().unwrap();

    state_guard.foo += 1;
    state_guard.bar -= 3;

    // update content using the updated state...
}

fn main() -> peacock::Result {
    let initial_state = MyState{
        foo: 0,
        bar: 1,
    };

    let mut app: ApplicationContext<MyState> = ApplicationContext::new_with_state(
        "Basic Peacock Usage",
        initial_state
    );

    app.read_xml_templates_auto()?;
    app.render_template_to_registry("index".into(), "index".into(), context!{})?;

    // lets assume that our 'index.xml' file contains a button element with id 'add_button'!
    app.register_message_receiver("add_button", Box::new(adder))

    app.run()
}
```

## Dynamic Content

Let's change the text content of the button with the `add_button` in response to the user
clicking on it! This requires knowing what the contents of the [`message::MessageGeneric`]
and [`message::MessageGenericInner`] are! `MessageGeneric` is just a
`(String, MessageGenericInner)`, and `MessageGenericInner` is an enum containing any information
a widget may need in order to satisfy its update conditions.

A button doesn't need any additional information to update the state because the event that occurs
(a click) is itself stateless. However, a widget such as a dropdown menu might require the new
selection to be passed in the message so it can properly update the state.

```rust
use peacock::{ApplicationContext, message::MessageGeneric, widget::{self, BoxedElementBuilder}};
use minijinja::context;

struct MyState {
    foo: usize,
    bar: i32,
}

fn adder(ctx: &mut ApplicationContext<MyState>, msg: MessageGeneric) {
    let state_handle = ctx.get_state();
    let mut state_guard = state_handle.write().unwrap();

    state_guard.foo += 1;
    state_guard.bar -= 3;

    // We can get the button using the id found in the MessageGeneric's
    // first field, and then get its children using the .get_children()
    // function of the ElementBuilder trait, which returns a vector of
    // child IDs. Buttons are constructed from a ButtonBuilder, which
    // can have exactly one child element. So we can index it by 0 to
    // get the id of the text content widget, and insert the new text
    // content by replacing the old text content with the newly
    // constructed text content.
    let button: BoxedElementBuilder<MyState> = ctx.get_widget(&msg.0).unwrap();
    let button_content_id: String = button.get_children()[0].clone();
    let button_content: Box<peacock::widget::text::TextBuilder> = widget::text::TextBuilder::new(
        format!("Foo is {}", state_guard.foo)
    );

    ctx.set_widget(button_content_id, button_content);
}

fn main() -> peacock::Result {
    let initial_state = MyState{
        foo: 0,
        bar: 1,
    };

    let mut app: ApplicationContext<MyState> = ApplicationContext::new_with_state(
        "Basic Peacock Usage",
        initial_state
    );

    app.read_xml_templates_auto()?;
    app.render_template_to_registry("index".into(), "index".into(), context!{})?;

    // lets assume that our 'index.xml' file contains a button element with id 'add_button'!
    app.register_message_receiver("add_button", Box::new(adder))

    app.run()
}
```
