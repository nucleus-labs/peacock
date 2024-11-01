
# Peacock

Peacock is a layout and styling framework written in Rust, designed to make the development of graphical
applications more intuitive and efficient. It emphasizes separation of concerns by allowing Designers to focus
on design aspects (layout, styling, and structure) while Engineers focus on functionality. This approach ensures
minimal overhead compared to frameworks like Qt, while providing a lightweight and powerful alternative.

# Core Functionality
- XML-based UI Structure
    - The core of Peacock is built around XML trees, where UI layouts are defined in a manner akin to writing
    HTML. It supports an in-memory XML tree structure through a core library, Pinion.
- CSS-based Styling
    - Peacock integrates a CSS parser using a core library, Crest, that allows for applying styles directly to
    XML elements, similar to how CSS works in web development.
- Custom Widgets
    - While Peacock is built on the iced framework, it introduces its own custom Widgets for more
    specialized integration with XML and CSS. These widgets are designed to mirror the flexibility
    of web components like `Row`, `Column`, and `Container`, ensuring seamless layout control without
    tying directly into iced’s default widget set.
- Templating Support
    - Peacock supports templating via the minijinja crate, enabling dynamic generation of UI components
    and layouts, further improving the flexibility for Designers to create reusable UI structures.
- Rust Integration
    - Although Peacock abstracts the UI layer through XML and CSS, it leverages Rust for interactivity and
    functionality, ensuring that Engineers can maintain full control over performance-critical aspects of
    the application.

Peacock is ideal for projects requiring a clear separation between design and development, with a focus on efficiency,
flexibility, and low overhead.
