# Profit Craftor

**Profit Craftor** is a tool developed during the Advanced Programming course held by prof. Patrignani of the University of Trento. This tool will be presented at the Software Faire, for anyone who may be interested in buying it.

## Description

This tool optimizes the sale of items to markets by choosing whether to convert some of them into others via the **craft** interface, with the aim of maximising profit.

## Features

These are the features we intend to add to the tool:

- automatically sell items to markets
- choose which items to sell and which ones to keep
- minimize the energy cost
- return specific errors based on the circumstances

## Work In Progress

We build this tool incrementally, by adding small capabilities, one at a time:

- [x] Pull the library from the register and have a running robot
- [x] Detect when the robot is near a market
- [ ] Sell everything the robot holds in its backpack
- [ ] Filter for what we want to sell and what we prefer not to
- [ ] Craft all the possible items from the ones in the backpack
- [ ] Optimize the crafting by choosing what items give more profit once crafted
- [ ] Perform the less expensive crafts first
- [ ] Craft items until a specific energy threshold is met

All the while we also need to take into consideration:

- [ ] Documentation
- [ ] Errors returned to the user
  - as of right now we consider returning a `LibError::OperationNotAllowed` when the robot is not near a `Market` and `LibError::NotEnoughEnergy` when the energy required for all the selling is more than what the robot has; subject tu future changes