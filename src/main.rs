use raylib::prelude::*;

struct Node {
    pub position: Vector2,
    pub old_position: Vector2,
}

impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            old_position: Vector2 { x, y },
        }
    }

    pub fn step(&mut self) {
        let temp: Vector2 = self.position;
        self.position += (self.position - self.old_position) + Vector2::new(0.0, 9.8) * 0.0167;
        self.old_position = temp;
    }
}

struct Rope {
    pub nodes: Vec<Node>,
    pub nodes_len: usize,
    pub node_distance: f32,
}

impl Rope {
    pub fn new(nodes_len: usize, x: f32, y: f32) -> Self {
        let node_distance: f32 = 10.0;
        let mut nodes: Vec<Node> = Vec::new();
        for i in 0..nodes_len {
            let node = Node::new(x, y + (i as f32 * node_distance));
            nodes.push(node);
        }

        Self {
            nodes: nodes,
            nodes_len: nodes_len,
            node_distance: node_distance,
        }
    }

    pub fn simulate(&mut self) {
        for i in self.nodes.iter_mut() {
            i.step();
        }
    }

    pub fn constraints(&mut self) {
        for i in 0..self.nodes_len - 1 {
            let diff_x = self.nodes[i].position.x - self.nodes[i+1].position.x;
            let diff_y = self.nodes[i].position.y - self.nodes[i+1].position.y;

            let distance =
                Vector2::distance_to(&self.nodes[i].position, self.nodes[i+1].position);
            let mut diff = 0.0;
            if distance > 0.0 {
                diff = (self.node_distance - distance) / distance;
            }
            let translate = Vector2::new(diff_x, diff_y) * (0.1 * diff);
            self.nodes[i].position += translate;
            self.nodes[i + 1].position -= translate;
        }
    }
}

fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Verlet Rope")
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut rope = Rope::new(10, (WIDTH / 2) as f32, (HEIGHT / 2) as f32);

    while !rl.window_should_close() {
        rope.simulate();

        const NUM_CONSTRAINT_ITERATIONS: usize = 50;
        for _ in 0..NUM_CONSTRAINT_ITERATIONS {
            rope.nodes[0].position = rl.get_mouse_position();
            rope.constraints();
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for i in 0..rope.nodes_len - 1 {
            d.draw_line_v(
                rope.nodes[i].position,
                rope.nodes[i + 1].position,
                Color::WHITE,
            );
        }
    }
}
