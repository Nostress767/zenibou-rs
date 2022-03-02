// TODO: add ALL VIRTUALKEY MACROS (VK_*) and UpdateKeyState for input
use core::cell::Cell;

unsafe extern "system" fn window_procedure(
    window_handle : winapi::shared::windef::HWND ,
    message : u32,
    w_param : usize,
    l_param : isize) -> isize {
    let window : *mut Window = winapi::um::winuser::GetWindowLongPtrW(window_handle, 0) as *mut Window;

    match message {
        winapi::um::winuser::WM_CREATE => {
            return 0;
        }
        winapi::um::winuser::WM_SIZE => {
            return 0;
        }
        winapi::um::winuser::WM_MOUSEMOVE => {
            if window as u32 != 0{
                (*window).mouse.is_focused.set(true);
                (*window).mouse.x = (l_param as u32 & 0x0000FFFF) as i16;
                (*window).mouse.y = (l_param as u32 & 0xFFFF0000) as i16;
            }
            return 0;
        }
        winapi::um::winuser::WM_MOUSELEAVE => {
            if window as u32 != 0{
                (*window).mouse.is_focused.set(false);
            }
            return 0;
        }
        winapi::um::winuser::WM_SETFOCUS => {
            if window as u32 != 0{
                (*window).is_focused = true;
            }
            return 0;
        }
        winapi::um::winuser::WM_KILLFOCUS => {
            if window as u32 != 0{
                (*window).is_focused = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_LBUTTONDOWN => {
            if window as u32 != 0{
                (*window).mouse.left_pressed.set(true);
            }
            return 0;
        }
        winapi::um::winuser::WM_LBUTTONUP => {
            if window as u32 != 0{
                (*window).mouse.left_pressed.set(false);
            }
            return 0;
        }
        winapi::um::winuser::WM_MBUTTONDOWN => {
            if window as u32 != 0{
                (*window).mouse.middle_pressed.set(true);
            }
            return 0;
        }
        winapi::um::winuser::WM_MBUTTONUP => {
            if window as u32 != 0{
                (*window).mouse.middle_pressed.set(false);
            }
            return 0;
        }
        winapi::um::winuser::WM_RBUTTONDOWN => {
            if window as u32 != 0{
                (*window).mouse.right_pressed.set(true);
            }
            return 0;
        }
        winapi::um::winuser::WM_RBUTTONUP => {
            if window as u32 != 0{
                (*window).mouse.right_pressed.set(false);
            }
            return 0;
        }
        winapi::um::winuser::WM_KEYUP => {
            //UpdateKeyState(w_param, l_param);
            return 0;
        }
        winapi::um::winuser::WM_SYSKEYDOWN  => {
            //UpdateKeyState(w_param, l_param);
            return 0;
        }
        winapi::um::winuser::WM_SYSKEYUP => {
            //UpdateKeyState(w_param, l_param);
            return 0;
        }
        winapi::um::winuser::WM_KEYDOWN => {
            //UpdateKeyState(w_param, l_param);
            return 0;
        }
        winapi::um::winuser::WM_DESTROY => {
            if window as u32 != 0{
                (*window).is_running = false;
                winapi::um::winuser::PostQuitMessage(0);
                winapi::um::winuser::DestroyWindow(window_handle);
                // NOTE: here, the window struct is not being dropped, but this should be fine
            }
            return 0;
        }
        _ => {
            winapi::um::winuser::DefWindowProcW(window_handle, message, w_param, l_param)
        }
    }   
}

#[cfg(feature = "std")]
pub fn start_engine(size_x : i32, size_y : i32, window : &mut Window){
    let title: Vec<u16> = window.title.encode_utf16().chain(Some(0)).collect();
    unsafe {
        winapi::um::winuser::SetProcessDpiAwarenessContext(winapi::shared::windef::DPI_AWARENESS_CONTEXT_SYSTEM_AWARE);
        window.instance = winapi::um::libloaderapi::GetModuleHandleW(0 as *const u16);
    }

    let class_name : Vec<u16> = String::from("Zenibou").encode_utf16().chain(Some(0)).collect();

    let mut window_class : winapi::um::winuser::WNDCLASSW = winapi::um::winuser::WNDCLASSW {
        style : winapi::um::winuser::CS_HREDRAW | winapi::um::winuser::CS_VREDRAW,
        lpfnWndProc : Some(window_procedure),
        cbClsExtra : 0,
        cbWndExtra : 4,
        hInstance : window.instance,
        hIcon : 0 as *mut winapi::shared::windef::HICON__,
        hCursor : 0 as *mut winapi::shared::windef::HICON__,
        hbrBackground : 0 as *mut winapi::shared::windef::HBRUSH__,
        lpszMenuName : 0 as *const u16,
        lpszClassName : class_name.as_ptr(),
    };
    
    unsafe {
        window_class.hCursor = winapi::um::winuser::LoadCursorW(0 as *mut winapi::shared::minwindef::HINSTANCE__,
            winapi::um::winuser::IDC_ARROW);
        winapi::um::winuser::ShowCursor(0);
        winapi::um::winuser::RegisterClassW(&window_class);
        
        window.current_pos_x = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) / 2) - (size_x / 2);
        window.current_pos_y = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) / 2) - (size_y / 2); 

        window.handle = winapi::um::winuser::CreateWindowExW(//winapi::um::winuser::WS_EX_OVERLAPPEDWINDOW,
                                    0, 
                                    class_name.as_ptr() as *const u16, title.as_ptr() as *const u16,
                                    winapi::um::winuser::WS_POPUP | winapi::um::winuser::WS_VISIBLE,
                                    window.current_pos_x, window.current_pos_y,
                                    size_x, size_y,
                                    0 as winapi::shared::windef::HWND, 0 as winapi::shared::windef::HMENU, 
                                    window.instance, 0 as winapi::shared::minwindef::LPVOID);
        winapi::um::winuser::SetWindowLongPtrW(window.handle, 0, (window as *const Window) as i32);
        window.bitmap_memory = winapi::um::memoryapi::VirtualAlloc(0 as *mut winapi::ctypes::c_void, size_x as usize * size_y as usize * 4, winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT, winapi::um::winnt::PAGE_READWRITE);
        window.bitmap_device_context = winapi::um::winuser::GetDC(window.handle);
    }

    window.width = size_x;
    window.height = size_y;

    window.bitmap_info = winapi::um::wingdi::BITMAPINFO { 
        bmiHeader : winapi::um::wingdi::BITMAPINFOHEADER { 
            biSize : core::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
            biWidth : size_x,
            biHeight : size_y,
            biPlanes : 1,
            biBitCount : 32,
            biCompression : winapi::um::wingdi::BI_RGB,
            biSizeImage : 0,
            biXPelsPerMeter : 0,
            biYPelsPerMeter : 0,
            biClrUsed : 0,
            biClrImportant : 0,
        },
        bmiColors : [
            winapi::um::wingdi::RGBQUAD {
                rgbBlue : 0,
                rgbGreen : 0,
                rgbRed : 0,
                rgbReserved : 0,
            }
        ],
    };
}

#[cfg(not(feature = "std"))]
pub fn start_engine(size_x : i32, size_y : i32, window : &mut Window){
    unsafe {
        winapi::um::winuser::SetProcessDpiAwarenessContext(winapi::shared::windef::DPI_AWARENESS_CONTEXT_SYSTEM_AWARE);
        window.instance = winapi::um::libloaderapi::GetModuleHandleW(0 as *const u16);
    }

    let class_name : [u16; 8] = ['Z' as u16,'e' as u16,'n' as u16,'i' as u16,'b' as u16,'o' as u16,'u' as u16,'\0' as u16];

    let mut window_class : winapi::um::winuser::WNDCLASSW = winapi::um::winuser::WNDCLASSW {
        style : winapi::um::winuser::CS_HREDRAW | winapi::um::winuser::CS_VREDRAW,
        lpfnWndProc : Some(window_procedure),
        cbClsExtra : 0,
        cbWndExtra : 4,
        hInstance : window.instance,
        hIcon : 0 as *mut winapi::shared::windef::HICON__,
        hCursor : 0 as *mut winapi::shared::windef::HICON__,
        hbrBackground : 0 as *mut winapi::shared::windef::HBRUSH__,
        lpszMenuName : 0 as *const u16,
        lpszClassName : class_name.as_ptr(),
    };
    
    unsafe {
        window_class.hCursor = winapi::um::winuser::LoadCursorW(0 as *mut winapi::shared::minwindef::HINSTANCE__,
            winapi::um::winuser::IDC_ARROW);
        winapi::um::winuser::ShowCursor(0);
        winapi::um::winuser::RegisterClassW(&window_class);
        
        window.current_pos_x = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) / 2) - (size_x / 2);
        window.current_pos_y = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) / 2) - (size_y / 2); 

        window.handle = winapi::um::winuser::CreateWindowExW(//winapi::um::winuser::WS_EX_OVERLAPPEDWINDOW,
                                    0, 
                                    class_name.as_ptr(), window.title.as_ptr(),
                                    winapi::um::winuser::WS_POPUP | winapi::um::winuser::WS_VISIBLE,
                                    window.current_pos_x, window.current_pos_y,
                                    size_x, size_y,
                                    0 as winapi::shared::windef::HWND, 0 as winapi::shared::windef::HMENU, 
                                    window.instance, 0 as winapi::shared::minwindef::LPVOID);
        winapi::um::winuser::SetWindowLongPtrW(window.handle, 0, (window as *const Window) as i32);
        window.bitmap_memory = winapi::um::memoryapi::VirtualAlloc(0 as *mut winapi::ctypes::c_void, size_x as usize * size_y as usize * 4, winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT, winapi::um::winnt::PAGE_READWRITE);
        window.bitmap_device_context = winapi::um::winuser::GetDC(window.handle);
    }

    window.width = size_x;
    window.height = size_y;

    window.bitmap_info = winapi::um::wingdi::BITMAPINFO { 
        bmiHeader : winapi::um::wingdi::BITMAPINFOHEADER { 
            biSize : core::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
            biWidth : size_x,
            biHeight : size_y,
            biPlanes : 1,
            biBitCount : 32,
            biCompression : winapi::um::wingdi::BI_RGB,
            biSizeImage : 0,
            biXPelsPerMeter : 0,
            biYPelsPerMeter : 0,
            biClrUsed : 0,
            biClrImportant : 0,
        },
        bmiColors : [
            winapi::um::wingdi::RGBQUAD {
                rgbBlue : 0,
                rgbGreen : 0,
                rgbRed : 0,
                rgbReserved : 0,
            }
        ],
    };
}

pub fn begin_frame(){
    unsafe {
        let mut msg = core::mem::MaybeUninit::uninit();
        while winapi::um::winuser::PeekMessageW(msg.as_mut_ptr(), 0 as *mut winapi::shared::windef::HWND__, 0, 0, winapi::um::winuser::PM_REMOVE) != 0 {
            winapi::um::winuser::TranslateMessage(msg.as_ptr());
            winapi::um::winuser::DispatchMessageW(msg.as_ptr());}
    }
}


impl Window {
    #[cfg(feature = "std")]
    pub fn end_frame(&self){
        unsafe{
            winapi::um::wingdi::StretchDIBits(
                        self.bitmap_device_context,
                        0,0,
                        self.width, self.height,
                        0,0,
                        self.width, self.height,
                        self.bitmap_memory,
                        &self.bitmap_info,
                        winapi::um::wingdi::DIB_RGB_COLORS,
                        winapi::um::wingdi::SRCCOPY);
        }
        //for(int32_t i = 0; i < 512; i++){
        //    if(Key[i].is_pressed){
        //        Key[i].is_pressed = false;
        //        Key[i].is_held = true;
        //        Key[i].is_released = false;}
        //    else if(Key[i].is_held){}
        //    else if(Key[i].is_released){
        //        Key[i].is_pressed = false;
        //        Key[i].is_held = false;
        //        Key[i].is_released = false;}}
        if self.mouse.left_pressed.get(){
           self.mouse.left_pressed.set(false);
        }
        self.clock.tick();
    }
    #[cfg(not(feature = "std"))]
    pub fn end_frame(&self){
        unsafe{
            winapi::um::wingdi::StretchDIBits(
                        self.bitmap_device_context,
                        0,0,
                        self.width, self.height,
                        0,0,
                        self.width, self.height,
                        self.bitmap_memory,
                        &self.bitmap_info,
                        winapi::um::wingdi::DIB_RGB_COLORS,
                        winapi::um::wingdi::SRCCOPY);
        }
        //for(int32_t i = 0; i < 512; i++){
        //    if(Key[i].is_pressed){
        //        Key[i].is_pressed = false;
        //        Key[i].is_held = true;
        //        Key[i].is_released = false;}
        //    else if(Key[i].is_held){}
        //    else if(Key[i].is_released){
        //        Key[i].is_pressed = false;
        //        Key[i].is_held = false;
        //        Key[i].is_released = false;}}
        if self.mouse.left_pressed.get(){
           self.mouse.left_pressed.set(false);
        }
    }

    pub fn d(&self, x : i32, y : i32, color : u32){
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            ()
        }
        let pixel : *mut u32 = self.bitmap_memory as *mut u32;
        // On zenibou its ARGB
        // But we default RGBA, so roll over byte
        unsafe { *pixel.offset((y * self.width + x) as isize) = (color << 24) | (color >> 8); }
    }
    pub fn c(&self, color : u32){
        let pixel : *mut u32 = self.bitmap_memory as *mut u32;
        let win_sz : i32 = self.width * self.height;
        for i in 0..win_sz{
            unsafe { *pixel.offset(i as isize) = (color << 24) | (color >> 8); }
        }
    }
}

pub struct Mouse{
    pub x : i16,
    pub y : i16,
    pub left_pressed : Cell<bool>,
    pub middle_pressed : Cell<bool>,
    pub right_pressed : Cell<bool>,
    pub left_held : Cell<bool>,
    pub middle_held : Cell<bool>,
    pub right_held : Cell<bool>,
    pub left_released : Cell<bool>,
    pub middle_released : Cell<bool>,
    pub right_released : Cell<bool>,
    pub is_focused : Cell<bool>,
}

#[derive(Copy, Clone)]
pub struct Key{
    pub is_pressed : bool,
    pub is_held : bool,
    pub is_released : bool,
}

#[allow(dead_code)]
enum MouseButtons {
  MouseLeft = 0, MouseRight = 1, MouseMiddle = 2
}

#[allow(dead_code)]
enum OtherKeys {
  Space = 32,
  Quotes = 39,
  Comma = 44, Minus, Period, FrontSlash,
  Semicolon = 59,
  Equal = 61,
  LeftBracket = 91, BackSlash, RightBracket,
  Backtick = 96,
  Escape = 128, Enter, Tab, Backspace, Insert, Delete, Right, Left, Down, Up, PageUp, PageDown, Home, End,
  Capslock, ScrollLock, NumLock, PrintScreen, PauseBreak,
  F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
  Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, Decimal, Divide, Multiply, Subtract, Add,
  LeftShift, LeftCtrl, LeftAlt, LeftSuper, RightShift, RightCtrl, RightAlt, RightSuper
}

pub struct Window{
    #[cfg(feature = "std")]
    pub title : String,

    #[cfg(not(feature = "std"))]
    pub title : [u16; 100],

    pub width : i32,
    pub height : i32,
    pub current_pos_x : i32,
    pub current_pos_y : i32,
    pub is_running : bool,
    pub is_focused : bool,

    #[cfg(feature = "std")]
    pub clock : Clock,

    pub mouse : Mouse,
    pub key : [Key; 256],
    pub bitmap_memory : *mut winapi::ctypes::c_void,
    pub bitmap_info : winapi::um::wingdi::BITMAPINFO,
    pub bitmap_device_context : winapi::shared::windef::HDC,
    pub handle : winapi::shared::windef::HWND,
    pub instance: winapi::shared::minwindef::HINSTANCE,
}

impl Default for Window {
    fn default () -> Window {
        Window{
            #[cfg(feature = "std")]
            title : String::new(),

            #[cfg(not(feature = "std"))]
            title : [0; 100],

            width : 0,
            height : 0,
            current_pos_x : 0,
            current_pos_y : 0,
            is_running : true,
            is_focused : true,

            #[cfg(feature = "std")]
            clock : Clock::default(),

            mouse : Mouse{
                x : 0,
                y : 0,
                left_pressed : Cell::new(false),
                middle_pressed : Cell::new(false),
                right_pressed : Cell::new(false),
                left_held : Cell::new(false),
                middle_held : Cell::new(false),
                right_held : Cell::new(false),
                left_released : Cell::new(false),
                middle_released : Cell::new(false),
                right_released : Cell::new(false),
                is_focused : Cell::new(false),
            },
            key : [Key{is_pressed : false, is_held : false, is_released : false}; 256],
            bitmap_memory : 0 as *mut winapi::ctypes::c_void,
            bitmap_info : winapi::um::wingdi::BITMAPINFO {
                bmiHeader : winapi::um::wingdi::BITMAPINFOHEADER {
                    biSize : core::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
                    biWidth : 0, biHeight : 0,
                    biPlanes : 1,
                    biBitCount : 32,
                    biCompression : winapi::um::wingdi::BI_RGB,
                    biSizeImage : 0,
                    biXPelsPerMeter : 0,biYPelsPerMeter : 0,
                    biClrUsed : 0, biClrImportant : 0,
                },
                bmiColors : [winapi::um::wingdi::RGBQUAD {
                    rgbBlue : 0,
                    rgbGreen : 0,
                    rgbRed : 0,
                    rgbReserved : 0,
                }],},
            bitmap_device_context : 0 as *mut winapi::shared::windef::HDC__,
            handle : 0 as *mut winapi::shared::windef::HWND__,
            instance: 0 as *mut winapi::shared::minwindef::HINSTANCE__,
        }
    }
}

#[cfg(feature = "std")]
extern crate libc;

#[cfg(feature = "std")]
extern {
    pub fn clock() -> ::libc::clock_t;
}

#[cfg(feature = "std")]
pub struct Clock{
    pub last_second_elapsed_time : Cell<f64>,
    pub last_frame_elapsed_time : Cell<f64>,
    pub total_elapsed_time : Cell<f64>,
    pub frame : Cell<u64>,
    pub frames_last_second : Cell<i32>,
    pub time1 : Cell<i32>,
    pub time2 : Cell<i32>,
}

#[cfg(feature = "std")]
impl Clock{
    fn tick(&self){
      self.time2.set(unsafe{ clock() });
      self.last_frame_elapsed_time.set(((self.time2.get() - self.time1.get()) as f64) / 1000.0);// / CLOCKS_PER_SEC;
      self.time1.set(self.time2.get());
      self.last_second_elapsed_time.set(self.last_second_elapsed_time.get() + self.last_frame_elapsed_time.get());
      self.total_elapsed_time.set(self.total_elapsed_time.get() + self.last_frame_elapsed_time.get());
      self.frame.set(self.frame.get() + 1);
      if self.last_second_elapsed_time.get() > 1.{
        self.frames_last_second.set(self.frame.get() as i32);
        self.last_second_elapsed_time.set(0.);
        self.frame.set(0);
      }
    }  
}

#[cfg(feature = "std")]
impl Default for Clock{
    fn default() -> Clock{
        Clock{
            last_second_elapsed_time : Cell::new(0.0),
            last_frame_elapsed_time : Cell::new(0.0),
            total_elapsed_time : Cell::new(0.0),
            frame : Cell::new(0),
            frames_last_second : Cell::new(0),
            time1 : Cell::new(0),
            time2 : Cell::new(0),
        }
    }
}
