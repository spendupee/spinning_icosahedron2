use ggez::{
    conf,
    event,
    graphics,
    Context,
    GameResult,
};
use mint;

const PHI: f32 =  1.618033988749895;

struct State {
    vertices: Vec<[f32; 3]>,
    edges: Vec<(usize, usize)>,
    angle_x: f32,
    angle_y: f32,
    d: f32,
    scale: f32,
}

impl State {
    fn new() -> GameResult<Self> {
        let vertices = vec![
            [0.0, 1.0, PHI],
            [0.0, -1.0, PHI],
            [0.0, 1.0, -PHI],
            [0.0, -1.0, -PHI],
            [1.0, PHI, 0.0],
            [-1.0, PHI, 0.0],
            [1.0, -PHI, 0.0],
            [-1.0, -PHI, 0.0],
            [PHI, 0.0, 1.0],
            [-PHI, 0.0, 1.0],
            [PHI, 0.0, -1.0],
            [-PHI, 0.0, -1.0],
        ];

        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            for j in (i + 1)..vertices.len() {
                let dx = vertices[i][0] - vertices[j][0];
                let dy = vertices[i][1] - vertices[j][1];
                let dz = vertices[i][2] - vertices[j][2];
                let dist_sq = dx * dx + dy * dy + dz * dz;
                if (dist_sq - 4.0).abs() < 1e-4 {
                    edges.push((i, j));
                }
            }
        }

        Ok(Self {
            vertices,
            edges,
            angle_x: 0.0,
            angle_y: 0.0,
            d: 3.0,
            scale: 200.0,
        })
    }
}

impl event::EventHandler<ggez::GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        const ROT_SPEED: f32 = 0.02;
        self.angle_x += ROT_SPEED;
        self.angle_y += ROT_SPEED;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let (screen_width, screen_height) = ctx.gfx.drawable_size();

        let mut projected = vec![[0.0; 2]; self.vertices.len()];

        for (i, v) in self.vertices.iter().enumerate() {
            // Apply Y rotation
            let rotated_y = [
                v[0] * self.angle_y.cos() + v[2] * self.angle_y.sin(),
                v[1],
                -v[0] * self.angle_y.sin() + v[2] * self.angle_y.cos(),
            ];

            // Apply X rotation
            let rotated = [
                rotated_y[0],
                rotated_y[1], //* self.angle_x.cos() - rotated_y[2] * self.angle_x.sin(),
                rotated_y[2], //* self.angle_x.sin() + rotated_y[2] * self.angle_x.cos(),
            ];

            // Perspective projection
            let z_depth = rotated[2] + self.d;
            let x_proj = (rotated[0] / z_depth) * self.scale + screen_width / 2.0;
            let y_proj = screen_height / 2.0 - (rotated[1] / z_depth) * self.scale;

            projected[i] = [x_proj, y_proj];
        }

        // Draw edges
        for &(i, j) in &self.edges {
            let start = projected[i];
            let end = projected[j];
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    mint::Point2 { x: start[0], y: start[1] },
                    mint::Point2 { x: end[0], y: end[1] },
                ],
                1.0,
                graphics::Color::WHITE,
            )?;
            canvas.draw(&line, graphics::DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("rotating_icosahedron", "ggez")
        .window_setup(conf::WindowSetup::default().title("Rotating Icosahedron"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;
    let state = State::new()?;
    event::run(ctx, event_loop, state)
}
