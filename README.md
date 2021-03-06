# Sokoban game
Rust implementation of sokoban game.
[tutorial](https://sokoban.iolivia.me/c01-00-intro.html)

## Game Engine

Game engine is it's a framework specific for game development. It provides tools for rendering, design, Ecs, game loop, etc...
[ggez](https://ggez.rs/).

## ECS

Entity Component System. Popular architectural pattern for game dev.
[Specs](https://specs.amethyst.rs/docs/tutorials/) ECS crate.

- Components: Data structures.
- Entities: Dummy container of one or more components.
- Systems: Behaviour over the components and entities.

# Steps

## 1.Classify Entities and Components.

### Entities

Sokoban is composed of 5 abstractions: Walls, Player, Floors, Boxes, and Box spots.

### Components

- Keep track of everything in our map: Position.
- Move character and boxes: Movement.
- We need to show something: Rendring.

### Apple + Pen

- Player entity: Position, Renderable, Movable
- Wall entity: Position, Renderable
- Floor entity: Position, Renderable
- Box entity: Position, Renderable, Movable
- Box spot entity: Position, Renderable
