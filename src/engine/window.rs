use crate::{camera::Camera, engine::timer::FrameTimer, prelude::*};

use anyhow::Result;
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub struct GameWindow {
    window: Arc<Window>,
    event_loop: Option<EventLoop<()>>,
}

impl GameWindow {
    pub fn new() -> Result<Self> {
        let event_loop = EventLoop::new()?;
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            #[allow(deprecated)]
            Arc::new(
                event_loop.create_window(
                    Window::default_attributes()
                        .with_title("DOOM")
                        .with_inner_size(size)
                        .with_min_inner_size(size),
                )?,
            )
        };

        Ok(Self {
            window,
            event_loop: Some(event_loop),
        })
    }

    pub fn run_event_loop(&mut self, camera: &mut Camera) -> Result<()> {
        let mut input = WinitInputHelper::new();

        println!("DOOM Engine Started");

        let mut timer = FrameTimer::new();

        #[allow(deprecated)]
        let res = self
            .event_loop
            .take()
            .unwrap()
            .run(|event, elwt| match event {
                Event::NewEvents(_) => input.step(),
                Event::WindowEvent { event, .. } => {
                    if input.process_window_event(&event) {
                        //
                    }

                    if event == WindowEvent::RedrawRequested {
                        timer.register_frame();
                    }
                }
                Event::AboutToWait => {
                    input.end_step();

                    // checking if the close command is requested
                    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                        elwt.exit();
                        return;
                    }

                    let (dx, _) = input.mouse_diff();
                    camera.rotate(dx as f64 * MOUSE_SENSIVITY);

                    while timer.should_update() {
                        let dt = timer.delta_time();

                        // camera movements
                        if input.key_held(KeyCode::KeyW) {
                            camera.move_forward(dt);
                        }
                        if input.key_held(KeyCode::KeyS) {
                            camera.move_backward(dt);
                        }
                        if input.key_held(KeyCode::KeyA) {
                            if input.held_alt() {
                                camera.strafe_left(dt);
                            } else {
                                camera.turn_left(dt);
                            }
                        }
                        if input.key_held(KeyCode::KeyD) {
                            if input.held_alt() {
                                camera.strafe_right(dt);
                            } else {
                                camera.turn_right(dt);
                            }
                        }

                        println!("window pos: {:?}", camera.position());

                        timer.step();
                    }

                    self.window.request_redraw();

                    elwt.set_control_flow(ControlFlow::WaitUntil(timer.get_next_frame()));
                }
                Event::DeviceEvent { event, .. } => {
                    input.process_device_event(&event);
                }
                _ => (),
            })?;

        Ok(res)
    }
}
