# utils

## viz.rs - Grid Visualizer

Real-time grid-based path planning visualizer using `minifb` + `plotters`.

### Types

- **`Point`** - `{ x: f32, y: f32 }` shared coordinate type
- **`GridViz`** - 800x800 window with internal RGB/u32 buffers

### API

```rust
use crate::utils::viz::{GridViz, Point};

let mut viz = GridViz::new("BFS", (min_x, max_x), (min_y, max_y));

viz.draw(&v_obs, pt_start, pt_goal, &v_explored, &v_path);

viz.wait_close(); // blocks until ESC or window close
```

### Usage Pattern

```rust
let mut viz = GridViz::new("BFS", (min_x, max_x), (min_y, max_y));
let mut v_explored = Vec::new();
let mut n = 0u32;

let v_path = plan(&grid, pt_start, pt_goal, |pt| {
    v_explored.push(pt);
    n += 1;
    if n % 10 == 0 {
        viz.draw(&v_obs, pt_start, pt_goal, &v_explored, &[]);
    }
});

viz.draw(&v_obs, pt_start, pt_goal, &v_explored, &v_path);
viz.wait_close();
```

### Draw Layers

1. Obstacles - black (r=2)
2. Explored - cyan (r=3)
3. Start - green (r=6)
4. Goal - blue (r=6)
5. Path - red line (w=2)
