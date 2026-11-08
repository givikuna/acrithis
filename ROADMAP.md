# Roadmap

This is a document that changes a lot.

## Status Legend

🟢 Done
🟡 In Progress
🔵 Planned
⚪ Idea / Untimed

## v0.0.1.x 🟡

- [x] Set up Nix Flake
- [x] Set up dev environment (including VSCode)
- [x] Basic directory structure
- [ ] Needs a way to be able to be set up declaratively
- [ ] Design a basic SQLite Wrapper
    - [ ] Uses a Nix subset language (as a DSL) for defining the Nodes (makes it easier to store proofs).
    - [ ] Enforces DAG
    - [ ] Has a proper Schema system
- [ ] Design basic Acrithis CLI

## v0.0.2.x 🔵

- [ ] Write first human axioms (Modus Ponens, Set Theory, ...)
- [ ] Set up a proper way for Lean programs to read from each other

## v0.0.3.x 🔵

- [ ] Create a methodology for allowing AI Agents to connect to Acrithis DB
- [ ] Create prompts for Agents (local and otherwise) to use when using Acrithis for their math proofs.

## v0.0.4.x 🔵

- [ ] WebUI for easy usage

## Parking Lot ⚪

- [ ] Find a way to sandbox AI agents
- [ ] Need a clean and easy way to store packages in the backend (could just use the `nix` language and derive statically)
- [ ] Need to find the right AI agent for this
- [ ] Need proper prompting strategies to force the model to generate the right output
- [ ] How to deal with different axiomatic frameworks and possible conflicts between them? Maybe some weird universe splitting?
