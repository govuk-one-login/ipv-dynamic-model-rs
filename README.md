Dynamic Journeys Playground
===========================

An interactive model of dynamic journeys for Identity Proving.

Goal
----

You can test the impact of traffic loads, outages and other problems users might face while proving their identity, on
the expected success rates.

Install and Run
---------------

You will need [Rust](https://rustup.rs/) and [cargo binstall](https://github.com/cargo-bins/cargo-binstall).

You can then run `make dependencies` to install everything else (which is just the dioxus-cli and its dependencies).

To run the program you can run `make serve` (which will simply run `dx serve` on the [ui-dioxus](/ui-dioxus) crate).

Usage
-----

You will need to provide your own `.yaml` file for creating the initial mapping.

Currently, you need to create a `test-data` directory, in the root of this project (it is `.gitignore`d), and place
your `test.yaml` in there. In the future we aim to have a more dynamic way to load the data.

Once running you simulate a specific user based on things like what documents they have, or how the service degrades
under load.

ToDo
----

- [x] Get `rowspan` from the `Row` object
- [x] Toggle services on and off in the UI
- [ ] Simulate user journeys, find all possible paths and their success rates, print combinations, their resulting 
      profile and individual success rate
- [ ] Simulate traffic load, update Services so they know if they are degraded or not
- [ ] Fix tests for `table_data` where we've used Signals
- [ ] More stuff I'm sure

Other Notes
-----------

### Why Rust

TL;DR: Rust was chosen for its accurate data models. 

We initially started this project in TypeScript but immediately hit some issues with data parsing and reasoning about
how the model would actually work.

Rusts consistency with types, complete prevention of inaccurate data models, and rich error handling made it a more
obvious choice.

Not having to worry about packages for types, testing, styles, linting, etc., was also nice as Rust has sensible
defaults for all of this from the go.

The downside to this approach is the language and ecosystem are less familiar for many.

### Important Dependencies

- To create the UI we used [Dioxus](https://dioxuslabs.com/). This allows us to create both app and web versions of this
  program.
- Parsing uses the [serde](https://serde.rs/) library. We chose YAML as the file format as its easy to read and write by
  hand.
- [anyhow](https://crates.io/crates/anyhow) and [thiserror](https://crates.io/crates/thiserror) are standard crates that
  trivialise error handling in Rust.
