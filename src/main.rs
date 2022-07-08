#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod zenibou;
use zenibou::begin_frame;

fn main(){
    let mut window : zenibou::Window = zenibou::Window::default();

    window.title = String::from("サンプル");

    zenibou::start_engine(600, 600, &mut window);

    let d = |x : i32, y : i32, col : u32| (&window).d(x, y, col);
    let c = |col : u32| (&window).c(col);
    let end_frame = || (&window).end_frame();

    while window.is_running {
        begin_frame();
            c(0xFFFF00FF);
            for i in 0..100{
                for j in 0..100{
                    d(100+j,100+i,0x00FF00FF);
                }
            }
            window.draw_string5x5(10, 430,
                format!("Frames last second: {} ({:.2})", window.clock.frames_last_second.get(), window.clock.total_elapsed_time.get()).as_str()
                , 0xFF0000FF, 3, 1, 5);
            window.draw_string5x5(50, 30, "Totally transformative elixir!", 0xFF0000FF, 3, 1, 5);
        end_frame();
    }
}
