# CyberNode

Did you ever want to just set up a small website without the hassle of a server,
but didn't want to have it all full of ads and didn't want to entrust it Github?
Wanted to use a secure connection like Tor, but didn't want to install it because
you're on another computer?
Or just wanted to play around with decentralized computing, but setting up and 
running an Ethereum, Avast, or anything else node was too complicated?

Meet CyberNode. This tool allows you to do all that and more straight out of
the browser:

- connecting to the network starts sharing your disk (10MB), your bandwidth
(1Mbps), and your CPU (1 CPU only)
- you get credits which you can use to:
  - store data in the network
  - communicate securely
  - run decentralized programs

In CyberNode, every browser connects with the other browsers to exchange messages
and retrieve data.
A blockchain makes sure that the sharing is done in a fair manner and nobody
profits more than they give.

## Roadmap

This is another effort to build such a system - this time called CyberNode.
The last time I started with the underlying protocol in [Fledger](https://fledg.re).
This time I start with the frontend.
Then I'll just need to make the two meet...

CyberNode is completely centralized but should offer as good a frontend
as possible:

- V0.1 - Backend and frontend communication: being connected, getting some Mana
while online, and simulation of nodes joining and leaving
- V0.2 - allow websites on the system: 'store' them on the joining nodes (only in
the backend), give some simple statistics about pages
- V0.3 - make it possible for users to add pages. Best would be a file manager
with a simple editor. And the possibility to drop files or a folder in the file
manager
- V0.4 - implement an actual sharing system with Mana allowing storage of pages,
and pages disappearing when Mana runs out. Or at lest put pages without Mana on a
"can be deleted" list

To be added:
- safe surfing (with some unsafe page blocking DNS in front, like [DNS0](dns0.eu))
- smart contract execution

Later:
- actually decentralize the system instead of simulating it with a centralized system

## Most important invariants in CyberNode

- runs in the web-browser first, on the server second:
  - easy to install - just browse to the URL
  - no set up required
  - still possible to run it on a server, if wanted
- fair sharing
  - contrary to other systems, Mana allows tracking how much you shared, and
  how much your shared content has been used
- sharing is caring
  - you only share things you support
  - opt-in to sharing by keeping it in the browser

# Why?

I love decentralized systems because they allow to run services on the internet
without a single point of failure, with a limited trusted third entity,
and they have some interesting privacy preserving attributes.

Unfortunately these systems are quite complex and long to build.
After having started on the backend, and then stopping because of missing time,
this shot is from the frontend.
The goals of CyberNode are:

- share my motivation
- find other people with the same interests
- test new technology

# Architecture

Cybernode is architectured as multiple Kademlia-routed networks which work together.

The main networks are the following three. 
One is common to all nodes, while the two others can in theory work together with other
similar networks.
However, in a first time, these three networks should be unique:
- common network for all nodes: group-id to node-id lookups
- preferred DNS network: readable names to group-id
- preferred Mana network: distributes and transfers Mana
- preferred reputation network: allows marking of other networks as NSFW or other
categories of undesirable content.

For usage, the following types of networks are available:
- web content: decentralized storage of webpages - if you view it, you opt-in to
also share the same content with other nodes.
- file storage: similar to IPFS, but each file storage network should conform to
a certain category of files.
This allows users to avoid networks they wouldn't want to support.
- smart contracts: automated execution of code which can transfer Mana

# Other such systems

Technically, CyberNode is based or similar to the following systems.
Feel free to create PRs to add other such Systems:

- [Bittorrent](https://en.wikipedia.org/wiki/BitTorrent) - for sharing data in a distributed system
- [Tor](https://www.torproject.org/) - anonymity online
- [Veilid](https://veilid.com/) - an Open Source, distributed application framework