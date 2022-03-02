#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod zenibou;
use zenibou::begin_frame;

// BEGIN std IMPLEMENTATION
#[cfg(feature = "std")]
fn main(){
    let mut window : zenibou::Window = zenibou::Window::default();

    window.title = String::from("サンプル");

    zenibou::start_engine(600, 600, &mut window);

    let d = |x : i32, y : i32, col : u32| (&window).d(x, y, col);
    let c = |col : u32| (&window).c(col);
    let end_frame = || (&window).end_frame();

    let mut seconds : u64 = 0;

    while window.is_running {
        begin_frame();
            if window.clock.total_elapsed_time.get() > seconds as f64{
                println!("Frames last second: {} ({})", window.clock.frames_last_second.get(), window.clock.total_elapsed_time.get());
                seconds += 1;
            }
            c(0xFFFF00FF);
            for i in 0..100{
                for j in 0..100{
                    d(100+j,100+i,0x00FF00FF);
                }
            }
        end_frame();
    }
}
// END std IMPLEMENTATION

// BEGIN no_std IMPLEMENTATION
#[cfg(not(feature = "std"))]
#[link(name = "vcruntime")]
extern {}

#[cfg(not(feature = "std"))]
#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let mut window : zenibou::Window = zenibou::Window::default();

    // TODO: find a better (easier) way to change the window title for no_std implementation
    window.title[0..4].copy_from_slice(&['サ' as u16,'ン' as u16,'プ' as u16,'ル' as u16]);

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
        end_frame();
    }
}

#[cfg(not(feature = "std"))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {loop {}}
// END no_std IMPLEMENTATION
