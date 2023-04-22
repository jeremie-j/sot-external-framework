use winapi::{
    shared::{minwindef::TRUE, windef::HWND},
    um::winuser::{
        GetWindowLongA, SendMessageA, SetLayeredWindowAttributes, SetWindowLongA, SetWindowPos,
        GWL_EXSTYLE, HTCAPTION, HWND_TOPMOST, SC_MOVE, SWP_NOMOVE, SWP_NOSIZE, WM_SYSCOMMAND,
        WS_EX_LAYERED, WS_EX_TRANSPARENT,
    },
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowExtWindows,
    window::{Fullscreen, WindowBuilder},
};

mod proc;

fn main() {
    // Create an event loop
    let event_loop = EventLoop::new();

    // Get the primary monitor for fullscreen
    let primary_monitor = event_loop.primary_monitor();

    // Create a transparent window
    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_fullscreen(Some(Fullscreen::Borderless(primary_monitor)))
        .build(&event_loop)
        .unwrap();

    // Set the window to be always on top using the winapi crate
    let hwnd = window.hwnd() as HWND;
    unsafe {
        SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);

        let ex_style = GetWindowLongA(hwnd, GWL_EXSTYLE);
        SetWindowLongA(
            hwnd,
            GWL_EXSTYLE,
            ex_style | WS_EX_LAYERED as i32 | WS_EX_TRANSPARENT as i32,
        );
        SetLayeredWindowAttributes(hwnd, 0, 128, 2);
    }

    let square_size = PhysicalSize::new(100, 100);
    let mut is_dragging = false;

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button, state, .. },
                ..
            } => {
                if button == MouseButton::Right {
                    is_dragging = !is_dragging;
                    if is_dragging {
                        // Start dragging the window
                        unsafe {
                            SendMessageA(hwnd, WM_SYSCOMMAND, SC_MOVE | HTCAPTION as usize, 0)
                        };
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                if is_dragging {
                    // Move the window with the mouse
                    let pos: PhysicalPosition<i32> = position.cast();
                    window.set_outer_position(pos);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
