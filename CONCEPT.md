# Bevy Jam 4: **That's a LOT of Entities!**

## Overall concept

Flattening the planet's ecosystem isn't the goal, it's just in the way of progress.

> "The Company thanks you for your participation in the [PlanetName] Environmental Rehabilitation Program. Your contribution has been logged."

**Themes:** deforestation, infestation, industry, growth, capitalism, space

**How to tie into theme:**
Massive number of

- trees
- pollen
- seed
- robots

On Screen

# Gameplay

## Core

- Land on a pre-generated planet
- The game runs a simulation for plant life
- Plants spread through various ways
  - along rivers
  - over the ocean
  - with the winds
  - through proximity without pollintation
  - on other plants
- Plants are harvested for power and minerals
- Minerals and power are exchanged for more harvesting power
- Players can spend resources and power to build/upgrade buildings
- Robots are used to collect resources

# Plants

## Life cycle

Plants have different phases, tied to different things.
All plants have 3 phases.

### 1. Growth

- Plants grow until they reach maturity
- Growth is based on parameters (resource availability/time/etc)
- Failure to achieve basic levels results in plant death

### 2. Pollination

- A mature plant will distribute pollen
- Distribution of pollen is based on parameters (time of year/weather/etc.)
- Pollen has a life span, after which, if it hasn't pollinated a plant, will despawn

### 3. Distribution

- Plants will then generate seeds which will be distributed similarly to pollen
- Same as pollen

### 4. Germination

- Seeds will only germinate if conditions are met (been in soil for long enough/attached to the right surface)
- Failure to meet conditions results in despawning

# Buildings

### - The Core

- This is the central node of your operations, protect this at all costs.

### - Distribution Tower

- Allows the distribution of resources/power.

### - Roboport

- Long range building that sends out robots to collect resources, robots return resources.

### - Logging Centre

- Short ranged building for harvesting.

### - Fan Tower

- Forces wind in a certain direction (protects against some weather events).

### - Shield Tower

- Prevents impacts from space.

### - Radar Tower

- Grants vision beyond what's granted by buildings.

### - Doppler Radar

- Provides vision of weather systems.

- Allows the player to see masses of cloud/rain/pollen/seeds before they're within vision.

# Art

2D top down, pixel graphics.

# MVP

## Basic graphics card processing system

- [ ] Pollen (particles/entities) should be able to move based on static wind
      system

## Basic plant

- [ ] Pollinates with AOE

- [ ] Distributes in AOE

- [ ] Germinates with time

- [ ] Grows with time

## Buildings

- [ ] Can place buildings

- [ ] Can delete buildings

- [ ] Buildings

  - [ ] The Core

  - [ ] Distribution Tower

  - [ ] Logging Centre

## UI

### Splash Screen

- [ ] Displays the logo

- [ ] Can be skipped with esc

- [ ] Moves to Main Menu on its own

### Main Menu

- [ ] Start Game

- [ ] Start Tutorial

- [ ] Map Seed

- [ ] Volume Slider

### Tutorial

- [ ] Shows how to place buildings

  - [ ] Must be within power range

- [ ] Shows how to extend power range

- [ ] How to delete buildings to regain resources

- [ ] How to repair and upgrade

- [ ] Bonus (Future)

  - [ ] Shows what happens if power dies

  - [ ] Shows how to defend against that

### Game

- [ ] Map navigation

- [ ] Basic click UI

- [ ] Keyboard shortcuts

- [ ] Timer

- [ ] Score (Plants remaining?)

- [ ] Pause Menu

## Structs

### TechTree

- TODO:

### Resource

- Energy
- Wood
- Water
- Mineral
- Ceramic
- Rare Metal

### Game State

- Time: f64
- Inventory: HashMap<[Resource](###Resource), i64>
- Tech Level: [TechTree](###TechTree)

### Plant

- Name: String
- Description: String
- Yield: [Resource](###Resource)[]

### Building

- Name: String
- Description: String
- Cost: [Resource](###Resource)[]
- Maintenence: [Resource](###Resource)[]
