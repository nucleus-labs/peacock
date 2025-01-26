
# Peacock
<!-- TODO: Add a banner image, icons for build results, status on crates.io, etc -->

## Overview

Peacock is a framework that's purpose-built to be as easy to pick up as possible for web and non-web
developers alike. It uses XML, CSS, and Rust to mirror the workflow of web development with XML and Rust
as stand-ins for HTML and JavaScript (respectively). Peacock is ideal for projects requiring a clear
separation between design and development.

## Status

Peacock is currently **far** from being ready for production applications. However, when Peacock *is* ready,
it will be an incredibly robust framework.

## Philosophy

The philosophy of Peacock is that of __accessibility__, __simplicity__, and __modularity__. Through
these, Peacock strives to be a framework deserving of its use in applications. It does this by enabling
intuitive design and efficient development by providing clear structures, reducing overhead, and
avoiding unnecessary complexity.

## Core Libraries

- Pinion (Templating Engine and XML Parsing) (Structure)
    - The core of Peacock is built around XML trees, where UI layouts are defined in a manner resembling
      HTML. It supports an in-memory XML tree structure through a core library, Pinion.
- Crest (CSS Parsing and Application) (Style)
    - Peacock integrates a CSS parser using a core library, Crest, that allows for applying styles
      directly to XML elements, similar to how CSS works in web development.

## Additional Features

- Templating Support
    - Instead of parsing XML directly, you can register it with a jinja2 template registry. Whenever
      you're ready to render it to XML, you just pass it the context and let peacock do the rest!
- Rust Integration
    - Although Peacock abstracts the UI layer through XML and CSS, it leverages Rust for interactivity
      and functionality so that engineers can maintain full control over performance-critical
      aspects of the application.

## Roadmap

- [ ] Peacock:  Documentation
- [ ] Peacock:  Inline style application using the Style attribute of DOM elements
- [ ] Crest:    DomElement trait that can be used to apply stylesheets and style rules and compare them
                against selectors
- [ ] Peacock:  Implement Crest's DomElement trait for each widget type
- [ ] Peacock:  Shared stylesheet application
- [ ] Peacock:  Signals for intelligently and efficiently updating widgets at runtime

## Quickstart

### Installation

Getting started with peacock is fairly straightforward! Peacock isn't published to crates.io since it's
not out of pre-alpha, so you can't just run a Cargo command (yet) (unfortunately).

`Cargo.toml`
```toml
[dependencies]
peacock = { git = "https://github.com/nucleus-labs/peacock", rev = "<rev>" }
minijinja = "2.5.0"
```

### Usage

`main.rs`
```rust
use peacock::ApplicationContext;
use minijinja::context;

fn main() -> peacock::Result {
    // () represents a stateless application, which is fine if you only have a static
    // application, or if relevant state is self-managed, such as by custom widgets.
    let mut app: ApplicationContext<()> = ApplicationContext::new("Basic Peacock App");

    // automatically search for xml files with the pattern 'static/xml/**/*.xml', then
    // add them to the template registry. 'home.xml' is added using the index 'home'
    app.read_xml_templates_auto()?;

    // pull the 'index' template and render it to XML, then parse the XML into widgets
    // in the widget registry using the 'index' key for the root element.
    app.render_template_to_registry("index".into(), "index".into(), context!{})?;

    app.run()
}
```

`index.xml`
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Button>Welcome to the basic Peacock example!</Button>
```

For more references, please check out the examples. They are written explicitly as a reference on
how to use peacock and are written with the intention of being as easy to follow as possible!

## Examples Guide

- Basic
  - creation of a managed application context
  - auto-discovery of xml files
  <!-- todo: auto-discovery of css files -->
  - rendering a template into the application context
  - running the application
- Dynamic Behaviour
  - creation of a managed application context
  - auto-discovery of xml files
  <!-- todo: auto-discovery of css files -->
  - rendering a template into the application context
  - subscribing to widget events
  - replacing content in response to widget events
  - running the application
- Dynamic Structure
  - creation of a managed application context
  - auto-discovery of xml files
  <!-- todo: auto-discovery of css files -->
  - rendering a template into the application context
  - procedurally rendering templates into xml component groups
    - eg. a card component for each member of a vector
  - running the application
