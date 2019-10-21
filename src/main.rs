#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

struct Paddle {
    pos: [f32; 2],
    width: f32,
    height: f32,
}

impl Paddle {
    fn move_h(&mut self, amt: f32) {
        let min = self.width / 2.0;
        let max = WIDTH as f32 - min;
        
        //~ if amt > 0.0 {
            //~ if self.pos[0] >= min {
                //~ self.pos[0] += amt;
            //~ }
            //~ if self.pos[0] < max {
                //~ self.pos[0] += amt;
            //~ }
        //~ } else {
            //~ if self.pos[0] > min {
                //~ self.pos[0] += amt;
            //~ }
            //~ if self.pos[0] <= max {
                //~ self.pos[0] += amt;
            //~ }
        //~ }
        
        let tmp = self.pos[0] + amt;
        if (tmp >= min) && (tmp <= max) {
            self.pos[0] = tmp;
        }
    }
}

struct Ball {
    pos: [f32; 2],
    width: f32,
    height: f32,
    v_x: f32,
    v_y: f32,
}

impl Ball {
    fn update(&mut self, paddle: &Paddle) {
        let min_x = self.width / 2.0;
        let max_x = WIDTH as f32 - min_x;
        
        let min_y = self.height / 2.0;
        let max_y = HEIGHT as f32 - min_y;
        
        if (self.pos[0] <= min_x) || (self.pos[0] >= max_x) {
            self.v_x = -self.v_x;
        }
        
        if (self.pos[1] <= min_y) || (self.pos[1] >= max_y) {
            self.v_y = -self.v_y;
        }
        
        let paddle_size_x = paddle.width / 2.0;
        //~ let paddle_max_x = WIDTH as f32 - paddle_min_x;
        
        let paddle_size_y = paddle.height / 2.0;
        //~ let paddle_max_y = HEIGHT as f32 - paddle_min_y;
        
        if (self.pos[0] >= paddle.pos[0] - paddle_size_x) && (self.pos[0] <= paddle.pos[0] + paddle_size_x) {
            //~ self.v_x = -self.v_x;
            if (self.pos[1] >= paddle.pos[1] - paddle_size_y) && (self.pos[1] <= paddle.pos[1] + paddle_size_y) {
                self.v_y = -self.v_y;
            }
        }
        
        //~ if (self.pos[1] >= paddle.pos[1] - paddle_size_y) && (self.pos[1] <= paddle.pos[1] + paddle_size_y) {
            //~ self.v_y = -self.v_y;
        //~ }
        
        self.pos[0] += self.v_x;
        self.pos[1] += self.v_y;
    }
}

fn main() {
    use glium::{glutin, Surface};
    //~ use std::io::Cursor;
    use glium::glutin::window::Fullscreen;
    //~ use cgmath::*;
    
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("GLium Test");
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
        //~ .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
    let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
    //~ for (i, video_mode) in monitor_handle.video_modes().enumerate() {
        //~ println!("Video mode #{}: {}", i, video_mode);
    //~ }
    let video_mode = monitor_handle.video_modes().nth(0).unwrap();
    
    //~ let fs = Fullscreen::Borderless(monitor_handle);
    let fs = Fullscreen::Exclusive(video_mode);
    display.gl_window().window().set_fullscreen(Some(fs));
    
    display.gl_window().window().set_cursor_grab(true).ok();
    display.gl_window().window().set_cursor_visible(false);
    
    //~ const image_set_names: &[&str] = &["../images/circle_01.png", "../images/circle_02.png", "../images/circle_03.png"];
    //~ let mut image_set: Vec<glium::texture::RawImage2d<'_, u8>> = Vec::new();
    
    //~ for i in image_set_names {
        //~ let image = image::open(i).unwrap().to_rgba();
        //~ let image_dimensions = image.dimensions();
        //~ image_set.push(glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions));
    //~ }
    
    let bg_01_image = image::open("/home/pi/Pictures/breakout/background_01.png").unwrap().to_rgba();
    let bg_01_dimensions = bg_01_image.dimensions();
    let bg_01_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&bg_01_image.into_raw(), bg_01_dimensions);
    let bg_01_tex = glium::texture::Texture2d::new(&display, bg_01_image).unwrap();
    
    let paddle_01_image = image::open("/home/pi/Pictures/breakout/paddle_01.png").unwrap().to_rgba();
    let paddle_01_dimensions = paddle_01_image.dimensions();
    let paddle_01_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&paddle_01_image.into_raw(), paddle_01_dimensions);
    let paddle_01_tex = glium::texture::Texture2d::new(&display, paddle_01_image).unwrap();
    
    let mut paddle1 = Paddle { pos: [160.0, 20.0], width: 320.0, height: 40.0 };
    
    let ball_01_image = image::open("/home/pi/Pictures/breakout/ball_01.png").unwrap().to_rgba();
    let ball_01_dimensions = ball_01_image.dimensions();
    let ball_01_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&ball_01_image.into_raw(), ball_01_dimensions);
    let ball_01_tex = glium::texture::Texture2d::new(&display, ball_01_image).unwrap();
    
    let mut ball1 = Ball { pos: [200.0, 100.0], width: 56.0, height: 56.0, v_x: 10.0, v_y: 15.0 };
    
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);
    
    let vertex0 = Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] };
    let vertex1 = Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] };
    let vertex2 = Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] };
    let vertex3 = Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] };
    
    let shape = vec![vertex0, vertex1, vertex2, vertex3];
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TriangleStrip, &[0 as u16, 1, 2, 3]).unwrap();
    
    let vertex_shader_src = r#"
        #version 120
        attribute vec2 position;
        attribute vec2 tex_coords;
        
        varying vec2 v_tex_coords;
        
        uniform mat4 perspective;
        uniform mat4 matrix;
       
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = perspective * matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    //~ let fragment_shader_src = r#"
        //~ #version 120
        //~ varying vec2 v_tex_coords;
        //~ uniform sampler2D tex_01;
        //~ uniform sampler2D tex_02;
        //~ uniform sampler2D tex_03;
        //~ uniform int tex_id;
        //~ void main() {
            //~ if (tex_id == 0) {
                //~ gl_FragColor = texture2D(tex_01, v_tex_coords);
            //~ } else if (tex_id == 1) {
                //~ gl_FragColor = texture2D(tex_02, v_tex_coords);
            //~ } else if (tex_id == 2) {
                //~ gl_FragColor = texture2D(tex_03, v_tex_coords);
            //~ }
        //~ }
    //~ "#;
    
    let fragment_shader_src = r#"
        #version 120
        varying vec2 v_tex_coords;
        uniform sampler2D tex;
        void main() {
            gl_FragColor = texture2D(tex, v_tex_coords);
        }
    "#;
    
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    let params = glium::DrawParameters {
        //~ depth: glium::Depth {
            //~ test: glium::DepthTest::IfLess,
            //~ write: true,
            //~ .. Default::default()
        //~ },
        blend: glium::draw_parameters::Blend::alpha_blending(),
        .. Default::default()
    };
    
    let ortho_mat = cgmath::ortho(0.0, WIDTH as f32, 0.0, HEIGHT as f32, -1.0, 1.0);
    let ortho = Into::<[[f32; 4]; 4]>::into(ortho_mat);
    
    //~ let mut val = 0.0;
    //~ let mut frame: i32 = 0;
    //~ let mut last_frame = std::time::Instant::now();
    
    let mut key_move_l = 0;
    let mut key_move_r = 0;
    
    event_loop.run(move |event, _, control_flow| {
        //~ let next_frame_time = std::time::Instant::now() +
            //~ std::time::Duration::from_nanos(16_666_667);
        //~ *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },
                glutin::event::WindowEvent::RedrawRequested => {
                    if key_move_l == 1 {
                        paddle1.move_h(-10.0);
                    }
                    if key_move_r == 1 {
                        paddle1.move_h(10.0);
                    }
                    
                    ball1.update(&paddle1);
                    
                    let mut target = display.draw();
                    target.clear_color(1.0, 1.0, 1.0, 1.0);
                    
                    //~ let uniforms1 = uniform! {
                        //~ perspective: ortho,
                        //~ matrix: [
                            //~ [320.0,   0.0, 0.0, 0.0],
                            //~ [  0.0, 320.0, 0.0, 0.0],
                            //~ [  0.0,   0.0, 1.0, 0.0],
                            //~ [500.0, 320.0, 1.0, 1.0f32],
                        //~ ],
                        //~ tex_01: &tex_01,
                        //~ tex_02: &tex_02,
                        //~ tex_03: &tex_03,
                        //~ tex_id: 2,
                    //~ };
                    //~ let uniforms2 = uniform! {
                        //~ perspective: ortho,
                        //~ matrix: [
                            //~ [320.0,   0.0, 0.0, 0.0],
                            //~ [  0.0, 320.0, 0.0, 0.0],
                            //~ [  0.0,   0.0, 1.0, 0.0],
                            //~ [  val, 520.0, 1.0, 1.0f32],
                        //~ ],
                        //~ tex_01: &tex_01,
                        //~ tex_02: &tex_02,
                        //~ tex_03: &tex_03,
                        //~ tex_id: frame,
                    //~ };
                    
                    let uniforms_bg = uniform! {
                        perspective: ortho,
                        matrix: [
                            [960.0,   0.0, 0.0, 0.0],
                            [  0.0, 540.0, 0.0, 0.0],
                            [  0.0,   0.0, 1.0, 0.0],
                            [960.0, 540.0, 0.0, 1.0f32],
                        ],
                        tex: &bg_01_tex,
                    };
                    
                    let uniforms_paddle1 = uniform! {
                        perspective: ortho,
                        matrix: [
                            [paddle1.width / 2.0,                  0.0, 0.0, 0.0],
                            [                0.0, paddle1.height / 2.0, 0.0, 0.0],
                            [                0.0,                  0.0, 1.0, 0.0],
                            [     paddle1.pos[0],      paddle1.pos[1], 0.0, 1.0f32],
                        ],
                        tex: &paddle_01_tex,
                    };
                    
                    let uniforms_ball1 = uniform! {
                        perspective: ortho,
                        matrix: [
                            [ball1.width / 2.0,                0.0, 0.0, 0.0],
                            [              0.0, ball1.height / 2.0, 0.0, 0.0],
                            [              0.0,                0.0, 1.0, 0.0],
                            [    ball1.pos[0],       ball1.pos[1], 0.0, 1.0f32],
                        ],
                        tex: &ball_01_tex,
                    };
                    
                    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms_bg, &params).unwrap();
                    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms_paddle1, &params).unwrap();
                    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms_ball1, &params).unwrap();
                    //~ target.draw(&vertex_buffer, &index_buffer, &program, &uniforms1, &params).unwrap();
                    //~ target.draw(&vertex_buffer, &index_buffer, &program, &uniforms2, &params).unwrap();
                    
                    target.finish().unwrap();
                    //~ display.gl_window().swap_buffers().unwrap();
                    
                    //~ val = (val + 1.0) % 1920.0;
                    //~ if val == 0.0 {
                        //~ frame = (frame + 1) % 3;
                    //~ }
                },
                glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_code, state) {
                    (glutin::event::VirtualKeyCode::Escape, _) => {
                        display.gl_window().window().set_cursor_grab(false).ok();
                        display.gl_window().window().set_cursor_visible(true);
                        *control_flow = glutin::event_loop::ControlFlow::Exit
                    },
                    (glutin::event::VirtualKeyCode::A, glutin::event::ElementState::Pressed) => {
                        key_move_l = 1;
                    },
                    (glutin::event::VirtualKeyCode::A, glutin::event::ElementState::Released) => {
                        key_move_l = 0;
                    },
                    //~ (glutin::event::VirtualKeyCode::S, _) => {
                        //~ frame = 1;
                    //~ },
                    (glutin::event::VirtualKeyCode::D, glutin::event::ElementState::Pressed) => {
                        key_move_r = 1;
                    },
                    (glutin::event::VirtualKeyCode::D, glutin::event::ElementState::Released) => {
                        key_move_r = 0;
                    },
                    _ => (),
                },
                _ => (),
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => (),
            },
            glutin::event::Event::EventsCleared => display.gl_window().window().request_redraw(),
            _ => (),
        }
    });
}
