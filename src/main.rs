#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod miniaudio;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod stb_image;

//#[allow(dead_code)]
mod zenibou;
use zenibou::JoystickButtons;

fn main(){
    let mut window : zenibou::Window = zenibou::Window::default();

    window.title = String::from("サンプル");
    zenibou::start_engine(600, 600, &mut window);

    window.declare_sound(include_bytes!("../Sound1.mp3"), "Test1\0");
    window.declare_sound(include_bytes!("../Sound2.mp3"), "Test2\0");

    window.alloc_music("Music", include_bytes!("../Music.mp3"), true);

    let mut monke = zenibou::alloc_sprite(include_bytes!("../monke.jpg"));

    while window.is_running {
        window.begin_frame();
            if window.key['a' as usize].is_pressed {
                window.music_start("Music");
            }
            else if window.key['s' as usize].is_pressed {
                window.music_seek("Music", 0);
            }
            else if window.key['d' as usize].is_pressed {
                window.music_stop("Music");
            }
            else if window.key['g' as usize].is_pressed {
                window.play_sound("Test1\0");
            }
            else if window.key['h' as usize].is_pressed {
                window.play_sound("Test2\0");
            }
            else if window.key['f' as usize].is_pressed {
                // TODO: fix this function on linux
                window.toggle_fullscreen();
            }
            else if window.key['r' as usize].is_pressed {
                window.set_window_size(320, 320);
            }
            else if window.key['t' as usize].is_pressed {
                window.set_window_size(800, 800);
            }
            else if window.key['p' as usize].is_pressed {
                window.is_running = false;
            }

            for i in 0..4 {
                if window.joystick[i as usize]
                    .button[JoystickButtons::LeftShoulder as usize]
                    .is_pressed {
                    window.play_sound("Test1\0");
                }
                else if window.joystick[i as usize]
                    .button[JoystickButtons::RightShoulder as usize]
                    .is_pressed {
                    window.play_sound("Test2\0");
                }
                else if window.joystick[i as usize]
                    .button[JoystickButtons::Start as usize]
                    .is_pressed {
                    // TODO: fix this function on linux
                    window.toggle_fullscreen();
                }
                else if window.joystick[i as usize]
                    .button[JoystickButtons::LeftThumb as usize]
                    .is_pressed {
                    window.set_window_size(320, 320);
                }
                else if window.joystick[i as usize]
                    .button[JoystickButtons::RightThumb as usize]
                    .is_pressed {
                    window.set_window_size(800, 800);
                }
                else if window.joystick[i as usize]
                    .button[JoystickButtons::Back as usize]
                    .is_pressed {
                    window.is_running = false;
                }
            }

            for i in 0..256 {
                if window.key[i as usize].is_pressed {
                    println!("The pressed key is {:x?} ('{}'). It was pressed on frame {}({})",
                        i, i, window.clock.frame, window.clock.total_elapsed_time);
                }
                //if window.key[i as usize].is_held {
                //    println!("The held key is {:x?} ('{}'). It was held on frame {}({})",
                //        i, i, window.clock.frame, window.clock.total_elapsed_time);
                //}
                if window.key[i as usize].is_released {
                    println!("The released key is {:x?} ('{}'). It was released on frame {}({})",
                        i, i, window.clock.frame, window.clock.total_elapsed_time);
                }
            }
            if window.mouse.left_pressed{
              println!("Mouse left pressed!");
            }
            if window.mouse.left_held{
              println!("Mouse left is being held");
            }
            if window.mouse.left_released{
              println!("Mouse left released!");
            }
            if window.mouse.middle_pressed{
              println!("Mouse middle pressed!");
            }
            if window.mouse.middle_held{
              println!("Mouse middle is being held");
            }
            if window.mouse.middle_released{
              println!("Mouse middle released!");
            }
            if window.mouse.right_pressed{
              println!("Mouse right pressed!");
            }
            if window.mouse.right_held{
              println!("Mouse right is being held");
            }
            if window.mouse.right_released{
              println!("Mouse right released!");
            }

            for i in 0..4 {
                for j in 0..32 {
                    if window.joystick[i as usize].button[j as usize].is_pressed{
                      println!("Controller {} pressed button {}. It was pressed on frame {}({})",
                             i, j, window.clock.frame, window.clock.total_elapsed_time);
                    }
                    //if window.joystick[i as usize].button[j as usize].is_held{
                    //  println!("Controller {} is holding button {}. It was held on frame {}({})",
                    //         i, j, window.clock.frame, window.clock.total_elapsed_time);
                    //}
                    if window.joystick[i as usize].button[j as usize].is_released{
                      println!("Controller {} released button {}. It was released on frame {}({})",
                             i, j, window.clock.frame, window.clock.total_elapsed_time);
                    }
                }
            }

            window.c(0xFFFF00);
            for i in 0..100{
                for j in 0..100{
                    window.d(100+j,100+i,0x00FF00);
                }
            }
            window.draw_string_5x6(10, window.height - 70,
                format!("Frames last second: {} ({:.2})", window.clock.frames_last_second, window.clock.total_elapsed_time).as_str()
                , 0xFF0000, 3, 1, 5);
            window.draw_string_5x5(50, window.height - 30, "Totally transformative elixir!", 0xFF0000, 3, 1, 5);
            window.draw_sprite(&monke);
        window.end_frame();
    }
    zenibou::free_sprite(&mut monke);
}
