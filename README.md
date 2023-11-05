# Bouncing Balls
Little physics simulation with a bunch of balls on a screen. Written in rust using ![Speedy2D library](https://github.com/QuantumBadger/Speedy2D).

Program contains 3 differennt types of simulation:
1. Standart - all circles are the same size and has equal mass. 
2. Masses - circles have different masses and sizes. Size depends on a mass of a circle
3. Velocity / Mass - color of a circle depends on it's velocity. Everything else is the same as in second type

Every type except third has an option to indicate a collision with

Due to poor collision-detection code, velocity of circles in every simuation is clamped by default. However they are still can get stuck.

Select a mode in code and run the simulation by setting a color of a circle on red for a small time.
```
cargo run
```

## Screenshots
- Mass-based simulation with colorful circles

![Bounsing Balls](https://github.com/LetY0uDown/Bouncing-balls/assets/93037214/39fad04e-fb30-4915-97a3-3c2da486f993)

- Velocity-based simulation. Slow balls are alomst transparent

![Bounsing Balls](https://github.com/LetY0uDown/Bouncing-balls/assets/93037214/63701327-2280-4379-9e9b-795d1ac25283)
