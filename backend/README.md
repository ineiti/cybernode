# The Backend

This is the backend for the cybernodes.
It is a simulation of the actual network.
But for simplification everything is centralized.
The goal is to have a simulation of the network, in order to focus on the frontend.
Over time, the actual node-code from `fledger` should be used.

The current implementation looks as follows:

- the web-frontend communicates with `Main` defined in [main.rs](./src/main.rs)
- `Main` starts a thread and listens to incoming requests
- the `Main` thread calls the `Broker` directly
- `Broker` has three modules which handle all communication:
  - `Network` to simulate the actual communication between the nodes.
  It also has the list of currently active nodes.
  - `Simulator` currently only makes nodes go online and offline.
  Later on it will also simulate actions.
  - `Web` gets messages from `Main` and communicates with the `Network` and
  the `Simulator`.
- `Node` is the actual definition of what happens in a node.
This part should later on be replaced by the actual `fledger`-nodes.

# Next Steps

## Small

- Add NodeID and Mana types

## Medium

- Create frontend

## Large

- Serve webpages
- Use actual nodes

# Changelog

2024-01-12
- Increase / decrease mana with every tick
  - done, make sure it's all correctly wired up with respect to Alive messages from the 
  simulator and the web.

2023-12-23
- Add names to nodes
- remove `online` field from `Node` - they should be added / removed using the `Trusted`
