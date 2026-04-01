# TODO
- [x] Rerwite the parser to be more performant (and reads frames)
- [x] Make changes the socket handling part to read data properly, instead of reading line [main.rs](src/main.rs#L55-L63)
    - [x] split the logic into a handler and listener part(need to understand signal handling for clean break in connections)

1/4
- [ ] Error handling to be verified
- [ ] come up with plan to properly handle for upcoming changes to introduce rraft for leaderless election


