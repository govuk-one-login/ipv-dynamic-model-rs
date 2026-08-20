User Journeys
=============

Dev Notes / Planning / Thinking Aloud
-------------------------------------

A user needs to progress through the CRIs to build up an identity profile.

The profile is constructed from GPG 45 scores, which the user can pick up from the CRIs.

We need to work out, for a set of X users per second:
- what are all possible paths through the CRIs
- what kind of profile each journey results in (if any)
- what CIs that journey could produce

To do this, we _could_ simulate every possible step through the system which would be nServices factorial... that'd be 
6,227,020,800 journeys based on 13 services :scream: Luckily not all of these journeys are possible or make sense.

To limit the possible journeys, we start by limiting how many first steps there are, then what each subsequent step
there is, producing a journey one step at a time. If a proportion of users could fail a journey, we can split the 
journey and continue on with the reduced number.

Likely, the easiest way to maintain the behavior is through a series of ranked rules. Ideally these rules would be user
editable.

There are two critical considerations though: 
- Do we care about finding the best route, and then what proportion of people are forced to take a less good route in
  order to succeed
- Or do we care about finding out the best way to distribute traffic without overloading one system, and going from 
  there

In the later case the total number of successful journeys would include overlaps in "success" as people could take
multiple routes to succeed. Both seem useful, so we could either somehow combine this or allow you to flip between them.

---

For now, I'm going to create a rule system that will take an existing journey (where they've been), and list of all
services, then return a new list whether its in a new order, or with some things filtered out. If any rule returns an
empty list, then that journey has failed.

Each next step will return a percent success giving us two journeys, one where the user successfully got something
from the CRI and one where they didn't.

---

Further work, we may need to provide more complex configuration around the CRIs for things like % of different scores,
% of CIs, more complex user config, etc.
