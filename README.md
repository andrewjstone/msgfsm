An example of a an enum based FSM that can handle messages and additionally returns output messages.

This provides all the capabilities of [funfsm](https://github.com/andrewjstone/funfsm) but has the
following advantages:

 * Much more idiomatic rust
 * Requires no match statements and therefore no default branches for unhandled messsages. Messages
   that aren't handled just return to the same state with a default implementation. The user only
   has to provide `Transition` implementations for handled messages.
 * Provides individual `next` functions for each message for a given state. This can be used to
   mimic function overloading as in Erlang
 * Different state data for each state, since the states are enums. This could be done in funfsm as
   well, but would require having the state function match the state variant of the ctx enum.
 * Allows straightforward decomposition of code by state or groups of states. They don't just look
   like functions anymore. Funfsm state functions could probably also be grouped together, but the
   shared context makes their decomposition harder. It's kind of weird to have to match on the
   ctx from within a state function when it can only handle a variant of the ctx (if implemented as
   in the previous bullet).
 * Requires no external libraries to use. Fsms are just state enums that implement the `Transition`
   trait.
 *


There are the following disadvantages, when compared to funfsm:
 * Uses trait specialization which is still not stabilized
 * Adds a bit more boilerplate to each state since now each state transition has to implement the
   Transition trait. This is opposed to a giant match in each state function, which may be just as
   ugly depending upon the reader's views.
 * Doesn't implement precondition and postcondition macros for individual states testing. However,
   these can be added easy enough.
