Dynamic Journeys Playground
===========================

An interactive model of dynamic journeys for Identity Proving.

Goal
----

You can test the impact of traffic loads, outages and other problems users might face while proving their identity, on
the expected success rates. 

Usage
-----

You will need to provide your own `.yaml` file for creating the initial mapping.

You can then simulate a specific user based on things like what documents they have, or how the service degrades under 
load.

ToDo
----

- [ ] Get `rowspan` from the `Row` object
- [ ] Simulate user journeys, find all possible paths and their success rates, print combinations, their resulting 
      profile and individual success rate
- [ ] Simulate traffic load, update Services so they know if they are degraded or not
- [ ] More stuff I'm sure

Other Notes
-----------

### Why Rust

TL;DR: Rust was chosen for its accurate data models. 

We initially started this project in TypeScript but immediately hit some issues with data parsing and reasoning about
how the model would actually work.
