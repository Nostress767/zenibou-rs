use crate::miniaudio;
use crate::stb_image;

use std::time;
use std::vec::Vec;
use std::mem::MaybeUninit;
use std::collections::HashMap;

// TODO: Try to find Rust safe alternatives for every "unsafe" part
// TODO: Finish this job

//void ProcessXEvent(void);

pub type Music = MaybeUninit<miniaudio::ma_sound>;

unsafe extern "system" fn window_procedure(
    window_handle : winapi::shared::windef::HWND ,
    message : u32,
    w_param : usize,
    l_param : isize) -> isize {
    let window : *mut Window = winapi::um::winuser::GetWindowLongPtrW(window_handle, 0) as *mut Window;

    match message {
        winapi::um::winuser::WM_SIZE => {
            if window as usize != 0{
                (*window).width = (l_param as u32 & 0x0000FFFF) as i32;
                (*window).height = ((l_param as u32 & 0xFFFF0000) >> 16) as i32;
            }
            return 0;
        }
        winapi::um::winuser::WM_MOVE => {
            if window as usize != 0{
                (*window).current_pos_x = (l_param & 0x0000FFFF) as i32;
                (*window).current_pos_y = (l_param & 0xFFFF0000 >> 16) as i32;
            }
            return 0;
        }
        winapi::um::winuser::WM_MOUSEMOVE => {
            if window as usize != 0{
                (*window).mouse.is_focused = true;
                (*window).mouse.x = (l_param as u32 & 0x0000FFFF) as i16;
                (*window).mouse.y = (l_param as u32 & 0xFFFF0000 >> 16) as i16;
            }
            return 0;
        }
        winapi::um::winuser::WM_MOUSELEAVE => {
            if window as usize != 0{
                (*window).mouse.is_focused = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_SETFOCUS => {
            if window as usize != 0{
                (*window).is_focused = true;
            }
            return 0;
        }
        winapi::um::winuser::WM_KILLFOCUS => {
            if window as usize != 0{
                (*window).is_focused = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_LBUTTONDOWN => {
            if window as usize != 0{
                (*window).mouse.left_pressed = true;
                (*window).mouse.left_held = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_LBUTTONUP => {
            if window as usize != 0{
                (*window).mouse.left_released = true;
                (*window).mouse.left_held = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_MBUTTONDOWN => {
            if window as usize != 0{
                (*window).mouse.middle_pressed = true;
                (*window).mouse.middle_held = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_MBUTTONUP => {
            if window as usize != 0{
                (*window).mouse.middle_released = true;
                (*window).mouse.middle_held = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_RBUTTONDOWN => {
            if window as usize != 0{
                (*window).mouse.right_pressed = true;
                (*window).mouse.right_held = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_RBUTTONUP => {
            if window as usize != 0{
                (*window).mouse.right_released = true;
                (*window).mouse.right_held = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_KEYUP |
        winapi::um::winuser::WM_SYSKEYDOWN |
        winapi::um::winuser::WM_SYSKEYUP |
        winapi::um::winuser::WM_KEYDOWN => {
            update_key_state(window_handle, w_param as i32, l_param as u32);
            return 0;
        }
        winapi::um::winuser::WM_CLOSE => {
            if window as usize != 0{
                (*window).is_running = false;
            }
            return 0;
        }
        winapi::um::winuser::WM_DESTROY => {
            if window as usize != 0{
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

pub fn start_engine(size_x : i32, size_y : i32, window : &mut Window){
//#ifndef _WIN32
//  // TODO: check if high DPI settings are needed
//  W.display = XOpenDisplay(0);
//
//  W.width = size_x;
//  W.height = size_y;
//
//  memset(W.name, 0, 4096);
//  memcpy(W.name, name, strlen(name));
//
//  W.handle = XCreateWindow(W.display, DefaultRootWindow(W.display), 0, 0,
//                           W.width, W.height, 0, CopyFromParent, CopyFromParent,
//                           W.visual, 0, NULL);
//
//  XSizeHints *size_hints = XAllocSizeHints();
//  size_hints->flags = PMinSize | PMaxSize;
//  size_hints->min_width = size_hints->max_width = W.width;
//  size_hints->min_height = size_hints->max_height = W.height;
//  XSetWMNormalHints(W.display, W.handle, size_hints);
//  XFree(size_hints);
//
//  XStoreName(W.display, W.handle, W.name);
//
//  // Unicode -> https://github.com/godotengine/godot/issues/2952
//  Xutf8SetWMProperties(W.display, W.handle, W.name, NULL, NULL, 0, NULL, NULL,
//                       NULL);
//
//  XSelectInput(W.display, W.handle,
//               FocusChangeMask | EnterWindowMask | LeaveWindowMask |
//                   KeyPressMask | KeyReleaseMask | ButtonPressMask |
//                   ButtonReleaseMask | PointerMotionMask);
//  XMapWindow(W.display, W.handle);
//
//  XWindowAttributes windowAttributes;
//  XGetWindowAttributes(W.display, W.handle, &windowAttributes);
//
//  W.bitmap_device_context = XCreateGC(W.display, W.handle, 0, 0);
//  W.bitmap_memory = malloc(W.width * W.height * 4);
//
//  W.bitmap =
//      XCreateImage(W.display, W.visual, windowAttributes.depth, ZPixmap, 0,
//                   (char *)W.bitmap_memory, W.width, W.height, 8 * 4, 0);
//#else
    let title: Vec<u16> = window.title.encode_utf16().chain(Some(0)).collect();
    unsafe {
        winapi::um::winuser::SetProcessDPIAware();
        window.instance = winapi::um::libloaderapi::GetModuleHandleW(0 as *const u16);
    }

    let class_name : Vec<u16> = String::from("Zenibou").encode_utf16().chain(Some(0)).collect();

    let mut window_class : winapi::um::winuser::WNDCLASSW = winapi::um::winuser::WNDCLASSW {
        style : winapi::um::winuser::CS_HREDRAW | winapi::um::winuser::CS_VREDRAW,
        lpfnWndProc : Some(window_procedure),
        cbClsExtra : 0,
        cbWndExtra : 8, // 4 for 32 bits, 8 for 64 bits
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
        //winapi::um::winuser::ShowCursor(0);
        winapi::um::winuser::RegisterClassW(&window_class);
        
        window.current_pos_x = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) / 2) - (size_x / 2);
        window.current_pos_y = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) / 2) - (size_y / 2); 

        let rect : winapi::shared::windef::RECT =
            winapi::shared::windef::RECT { left: 0, top: 0, right: size_x, bottom: size_y };
        winapi::um::winuser::AdjustWindowRectEx((&rect as *const winapi::shared::windef::RECT) as *mut winapi::shared::windef::RECT,
            winapi::um::winuser::WS_CAPTION | winapi::um::winuser::WS_VISIBLE | winapi::um::winuser::WS_SYSMENU |
            winapi::um::winuser::WS_MAXIMIZEBOX | winapi::um::winuser::WS_MINIMIZEBOX,
            //winapi::um::winuser::WS_OVERLAPPEDWINDOW | winapi::um::winuser::WS_VISIBLE,
            0,
            0);

        window.handle = winapi::um::winuser::CreateWindowExW(//winapi::um::winuser::WS_EX_OVERLAPPEDWINDOW,
                                    0, 
                                    class_name.as_ptr() as *const u16, title.as_ptr() as *const u16,
                                    winapi::um::winuser::WS_CAPTION | winapi::um::winuser::WS_VISIBLE | winapi::um::winuser::WS_SYSMENU,
                                    //winapi::um::winuser::WS_OVERLAPPEDWINDOW | winapi::um::winuser::WS_VISIBLE,
                                    //winapi::um::winuser::WS_POPUP | winapi::um::winuser::WS_VISIBLE,
                                    window.current_pos_x, window.current_pos_y,
                                    rect.right - rect.left, rect.bottom - rect.top,
                                    0 as winapi::shared::windef::HWND, 0 as winapi::shared::windef::HMENU, 
                                    window.instance, 0 as winapi::shared::minwindef::LPVOID);
        winapi::um::winuser::SetWindowLongPtrW(window.handle, 0, (window as *const Window) as isize);
        window.bitmap_memory = winapi::um::memoryapi::VirtualAlloc(0 as *mut winapi::ctypes::c_void, size_x as usize * size_y as usize * 4, winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT, winapi::um::winnt::PAGE_READWRITE);
        window.bitmap_device_context = winapi::um::winuser::GetDC(window.handle);
    }
    window.initialize_audio();

    window.width = size_x;
    window.height = size_y;

    window.bitmap_info = winapi::um::wingdi::BITMAPINFO { 
        bmiHeader : winapi::um::wingdi::BITMAPINFOHEADER { 
            biSize : std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
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

impl Window {
    pub fn begin_frame(&mut self){
        unsafe { self.update_joystick_state() };
        //#ifndef _WIN32
        //  while (XPending(W.display) > 0) {
        //    XNextEvent(W.display, &W.msg);
        //    ProcessXEvent();
        //  }
        //#else
        //  while (PeekMessage(&W.msg, NULL, 0, 0, PM_REMOVE)) {
        //    TranslateMessage(&W.msg);
        //    DispatchMessage(&W.msg);
        //  }
        //#endif
        unsafe {
            let mut msg = std::mem::MaybeUninit::uninit();
            while winapi::um::winuser::PeekMessageW(msg.as_mut_ptr(), 0 as *mut winapi::shared::windef::HWND__, 0, 0, winapi::um::winuser::PM_REMOVE) != 0 {
                winapi::um::winuser::TranslateMessage(msg.as_ptr());
                winapi::um::winuser::DispatchMessageW(msg.as_ptr());}
        }
    }
    pub fn end_frame(&mut self){
        //#ifndef _WIN32
        //  XPutImage(W.display, W.handle, W.bitmap_device_context, W.bitmap, 0, 0, 0, 0,
        //            W.bitmap->width, W.bitmap->height);
        //#else
        //  StretchDIBits(W.bitmap_device_context, W.offset_x, W.offset_y, W.width,
        //                W.height, 0, 0, W.width, W.height, W.bitmap_memory,
        //                &W.bitmap_info, DIB_RGB_COLORS, SRCCOPY);
        //#endif
        for i in 0..4 {
          for j in 0..32 {
            if self.joystick[i].button[j].is_pressed{
              self.joystick[i].button[j].is_held = true;
            }
            self.joystick[i].button[j].is_pressed = false;
            self.joystick[i].button[j].is_released = false;
          }
        }
        unsafe{
            // TODO: fix maximize window
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

        for i in 0..256 {
          if self.key[i].is_pressed{
            self.key[i].is_held = true;
          }
          self.key[i].is_pressed = false;
          self.key[i].is_released = false;
        }
        
        self.key[OtherKeys::LeftShift  as usize].is_held = false;
        self.key[OtherKeys::LeftCtrl   as usize].is_held = false;
        self.key[OtherKeys::LeftAlt    as usize].is_held = false;
        self.key[OtherKeys::LeftSuper  as usize].is_held = false;
        self.key[OtherKeys::RightShift as usize].is_held = false;
        self.key[OtherKeys::RightCtrl  as usize].is_held = false;
        self.key[OtherKeys::RightAlt   as usize].is_held = false;
        self.key[OtherKeys::RightSuper as usize].is_held = false;
        
        if self.mouse.left_pressed{
          self.mouse.left_held = true;
        }
        if self.mouse.middle_pressed{
          self.mouse.middle_held = true;
        }
        if self.mouse.right_pressed{
          self.mouse.right_held = true;
        }
        
        self.mouse.left_pressed = false;
        self.mouse.left_released = false;
        self.mouse.middle_pressed = false;
        self.mouse.middle_released = false;
        self.mouse.right_pressed = false;
        self.mouse.right_released = false;

        self.clock.tick();
    }
    pub fn draw_string_5x6(&self, x : i32, y : i32, text : &str, col : u32, scal : i32, spc_x : i32, spc_y : i32){
        self.draw_font_sprite(&self.internal_font_5x6, 5, 6, x, y, text, col, scal, spc_x, spc_y);
    }
    pub fn draw_string_5x5(&self, x : i32, y : i32, text : &str, col : u32, scal : i32, spc_x : i32, spc_y : i32){
        self.draw_font_sprite(&self.internal_font_5x5, 5, 5, x, y, text, col, scal, spc_x, spc_y);
    }
    pub fn toggle_fullscreen(&mut self) {
        unsafe{
        //#ifndef _WIN32
        //  // TODO: make this (function) work
        //  // I have no idea how to do this
        //  if (W.is_fullscreen) {
        //    SetWindowSize(W.previous_width, W.previous_height);
        //    W.is_fullscreen = false;
        //  } else {
        //    W.previous_width = W.width;
        //    W.previous_height = W.height;
        //    SetWindowSize(DisplayWidth(W.display, DefaultScreen(W.display)),
        //                  DisplayHeight(W.display, DefaultScreen(W.display)));
        //    W.is_fullscreen = true;
        //  }
        //#else
        
        if self.is_fullscreen {
            self.is_fullscreen = false;
            self.set_window_size(self.previous_width, self.previous_height);
          // SetWindowLongPtr(W.handle, GWL_STYLE,
          //                  WS_CAPTION | WS_VISIBLE | WS_SYSMENU);
          // RECT rect = {0, 0, W.previous_width, W.previous_height};
          // AdjustWindowRectEx(&rect, WS_OVERLAPPEDWINDOW | WS_VISIBLE, false, 0);
          // MoveWindow(W.handle, (GetSystemMetrics(SM_CXSCREEN) / 2) -
          // (W.previous_width / 2),
          //            (GetSystemMetrics(SM_CYSCREEN) / 2) - (W.previous_height / 2),
          //            rect.right - rect.left, rect.bottom - rect.top, true);
        } else {
            self.is_fullscreen = true;
            self.previous_width = self.width;
            self.previous_height = self.height;
            self.set_window_size(
                winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN),
                winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN));
          // SetWindowLongPtr(W.handle, GWL_STYLE, WS_POPUP | WS_VISIBLE);
          // MoveWindow(W.handle, 0, 0, GetSystemMetrics(SM_CXSCREEN),
          //            GetSystemMetrics(SM_CYSCREEN), true);
        }
        }
    }
    pub fn set_window_size(&mut self, size_x : i32, size_y : i32){
        unsafe{
    //#ifndef _WIN32
    //  XDestroyImage(W.bitmap);
    //  XDestroyWindow(W.display, W.handle);
    //
    //  W.width = size_x;
    //  W.height = size_y;
    //
    //  W.handle = XCreateWindow(W.display, DefaultRootWindow(W.display), 0, 0,
    //                           W.width, W.height, 0, CopyFromParent, CopyFromParent,
    //                           W.visual, 0, NULL);
    //
    //  XStoreName(W.display, W.handle, W.name);
    //  Xutf8SetWMProperties(W.display, W.handle, W.name, NULL, NULL, 0, NULL, NULL,
    //                       NULL);
    //
    //  XSelectInput(W.display, W.handle,
    //               KeyPressMask | KeyReleaseMask | ButtonPress | ButtonRelease |
    //                   MotionNotify);
    //  XMapWindow(W.display, W.handle);
    //
    //  XWindowAttributes windowAttributes;
    //  XGetWindowAttributes(W.display, W.handle, &windowAttributes);
    //
    //  W.bitmap_device_context = XCreateGC(W.display, W.handle, 0, 0);
    //  W.bitmap_memory = malloc(W.width * W.height * 4);
    //
    //  XSizeHints *size_hints = XAllocSizeHints();
    //  size_hints->flags = PMinSize | PMaxSize;
    //  size_hints->min_width = size_hints->max_width = W.width;
    //  size_hints->min_height = size_hints->max_height = W.height;
    //  XSetWMNormalHints(W.display, W.handle, size_hints);
    //  XFree(size_hints);
    //
    //  W.bitmap =
    //      XCreateImage(W.display, W.visual, windowAttributes.depth, ZPixmap, 0,
    //                   (char *)W.bitmap_memory, W.width, W.height, 8 * 4, 0);
    //#else

    winapi::um::memoryapi::VirtualFree(self.bitmap_memory, 0, winapi::um::winnt::MEM_RELEASE);
    self.bitmap_memory = winapi::um::memoryapi::VirtualAlloc(0 as *mut winapi::ctypes::c_void,
        size_x as usize * size_y as usize * 4,
        winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT,
        winapi::um::winnt::PAGE_READWRITE);
    
    winapi::um::winuser::ReleaseDC(self.handle, self.bitmap_device_context);
    self.bitmap_info = winapi::um::wingdi::BITMAPINFO { 
        bmiHeader : winapi::um::wingdi::BITMAPINFOHEADER { 
            biSize : std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
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
    self.bitmap_device_context = winapi::um::winuser::GetDC(self.handle);
    
    let rect : winapi::shared::windef::RECT =
        winapi::shared::windef::RECT { left: 0, top: 0, right: size_x, bottom: size_y };
    if size_x > winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) ||
        size_y > winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN){
        return;
    }
    else if size_x == winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) &&
            size_y == winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) {
        winapi::um::winuser::SetWindowLongPtrW(self.handle,
            winapi::um::winuser::GWL_STYLE,
            (winapi::um::winuser::WS_POPUP |
            winapi::um::winuser::WS_VISIBLE) as isize);
        winapi::um::winuser::AdjustWindowRectEx((&rect as *const winapi::shared::windef::RECT) as *mut winapi::shared::windef::RECT,
            winapi::um::winuser::WS_POPUP |
            winapi::um::winuser::WS_VISIBLE, 0, 0);
    } else {
        winapi::um::winuser::SetWindowLongPtrW(self.handle,
            winapi::um::winuser::GWL_STYLE,
            (winapi::um::winuser::WS_CAPTION |
            winapi::um::winuser::WS_VISIBLE |
            winapi::um::winuser::WS_SYSMENU) as isize);
        winapi::um::winuser::AdjustWindowRectEx((&rect as *const winapi::shared::windef::RECT) as *mut winapi::shared::windef::RECT,
            winapi::um::winuser::WS_CAPTION |
            winapi::um::winuser::WS_VISIBLE |
            winapi::um::winuser::WS_SYSMENU, 0, 0);
    }
    self.current_pos_x = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) / 2) - (size_x / 2);
    self.current_pos_y = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) / 2) - (size_y / 2);
    winapi::um::winuser::MoveWindow(self.handle, self.current_pos_x, self.current_pos_y, rect.right - rect.left,
               rect.bottom - rect.top, 1);
        }
    }

    pub fn draw_font_sprite(&self, spr : &Sprite, size_x : i32, size_y : i32, x : i32, y : i32, text : &str, color : u32, scale : i32, space_x : i32, space_y : i32){
        let (mut text_x, mut text_y) = (0i32, 0i32);
        let str_len : usize = text.len();
        for i in 0..str_len {
            let ith_char : char = text.chars().nth(i).unwrap();
            if ith_char == '\n' {
              text_x = 0; text_y -= size_y * scale + space_y;
            }
            else {
                let char_index_x : i32 = (ith_char as i32 - 32) % 16;
                let char_index_y : i32 = (ith_char as i32 - 32) / 16;
                if scale > 1 {
                    for j in 0i32..size_x {
                        for k in 0i32..size_y {
                            if spr.data[(k + char_index_y * size_y) as usize][((j + char_index_x * size_x) * 4) as usize] != 0 {
                                for scaled_i in 0i32..scale {
                                    for scaled_j in 0i32..scale {
                                        Self::d(self, x + text_x + (j * scale) + scaled_i, y + text_y + (k * scale) + scaled_j, color);
                                    }
                                }
                            }
                        }
                    }
                }
                else {
                    for j in 0i32..size_x {
                        for k in 0i32..size_y {
                            if spr.data[(k + char_index_y * size_y) as usize][((j + char_index_x * size_x) * 4) as usize] != 0 {
                                Self::d(self, x + text_x + j, y + text_y + k, color);
                            }
                        }
                    }
                }
                text_x += (size_x + space_x) * scale as i32;
            }
        }
    }

    pub fn draw_sprite(&self, spr : &Sprite){
        for i in 0..spr.height {
            for j in 0..spr.width {
                self.d(spr.x + j, spr.y + i,
                    (spr.data[i as usize][(j * 4) as usize] as u32) << 16 |
                    (spr.data[i as usize][(j * 4 + 1) as usize] as u32) << 8 |
                    (spr.data[i as usize][(j * 4 + 2) as usize] as u32) |
                    (spr.data[i as usize][(j * 4 + 3) as usize] as u32) << 24);
            }
        }
    }

    pub fn initialize_audio(&mut self){
        unsafe{
            miniaudio::ma_engine_init(0 as *const miniaudio::ma_engine_config, self.audio_engine.as_mut_ptr())
        };
    }

    pub fn play_sound(&mut self, name : &str){
        unsafe{
            miniaudio::ma_engine_play_sound(self.audio_engine.as_mut_ptr(), name.as_ptr() as *const i8, 0 as *mut miniaudio::ma_sound)
        };
    }

    pub fn alloc_music(&mut self, name : &'static str, data : &[u8], loops : bool){
        self.audio_decoders.insert(name, MaybeUninit::uninit());
        
        unsafe{
            miniaudio::ma_decoder_init_memory(
                data.as_ptr() as *const std::ffi::c_void,
                data.len() as u64, self.decoder_config.as_ptr(),
                (*self.audio_decoders.get_mut(name).unwrap()).as_mut_ptr())
        };
        
        self.musics.insert(name, MaybeUninit::uninit());
        unsafe{ miniaudio::ma_sound_init_from_data_source(
            self.audio_engine.as_mut_ptr(),
            (*self.audio_decoders.get_mut(name).unwrap()).as_mut_ptr() as *mut std::ffi::c_void,
            miniaudio::ma_sound_flags_MA_SOUND_FLAG_DECODE as u32,
            0 as *mut miniaudio::ma_sound,
            (*self.musics.get_mut(name).unwrap()).as_mut_ptr()) };

        if loops {
          unsafe{ miniaudio::ma_sound_set_looping(
              (*self.musics.get_mut(name).unwrap()).as_mut_ptr(),
              miniaudio::MA_TRUE)
          };
        }
    }
    pub fn music_start(&mut self, music : &str){
        unsafe{ miniaudio::ma_sound_start(
            (*self.musics.get_mut(music).unwrap()).as_mut_ptr())
        };
    }

    pub fn music_stop(&mut self, music : &str){
        unsafe{ miniaudio::ma_sound_stop(
            (*self.musics.get_mut(music).unwrap()).as_mut_ptr())
        };
    }

    pub fn music_seek(&mut self, music : &str, frame: u64){
        unsafe{ miniaudio::ma_sound_seek_to_pcm_frame(
            (*self.musics.get_mut(music).unwrap()).as_mut_ptr(), frame)
        };
    }

    pub fn declare_sound(&mut self, data : &[u8], name : &str){
        unsafe{
            miniaudio::ma_resource_manager_register_encoded_data(
                miniaudio::ma_engine_get_resource_manager(self.audio_engine.as_mut_ptr()),
                name.as_ptr() as *const i8,
                data.as_ptr() as *const std::ffi::c_void, data.len() as u64
            );
        };
    }

    pub unsafe fn update_joystick_state(&mut self){
    //#ifndef _WIN32
    //  for (i8 i = 0; i < 4; i++) {
    //    if (!self.joystick[i].is_on &&
    //        (self.joystick[i].fd = open(self.joystick[i].path, O_NONBLOCK)) > 0) {
    //      self.joystick[i].is_on = true;
    //      ioctl(self.joystick[i].fd, JSIOCGAXES, &self.joystick[i].axes_n);
    //      ioctl(self.joystick[i].fd, JSIOCGBUTTONS, &self.joystick[i].buttons_n);
    //      ioctl(self.joystick[i].fd, JSIOCGNAME(128), self.joystick[i].name);
    //      // printf("%s was connected on port %d\n", self.joystick[i].name, i);
    //      // printf("It has %d buttons and %d axes\n", self.joystick[i].buttons_n,
    //      //        self.joystick[i].axes_n);
    //    }
    //  }
    //  for (i8 i = 0; i < 4; i++) {
    //    if (!self.joystick[i].is_on)
    //      continue;
    //    // TODO: clean this up
    //    // Since windows uses the enum directly, it could be re-ordered to fit linux
    //    // as ints to avoid switch
    //    // e.g.: as has already been done with the axes
    //    while (read(self.joystick[i].fd, &self.joystick[i].event, sizeof(self.joystick[i].event)) >
    //           0) {
    //      switch (self.joystick[i].event.type) {
    //      case JS_EVENT_BUTTON: {
    //        if (self.joystick[i].event.value) {
    //          i32 mapped_button = 0;
    //          switch (self.joystick[i].event.number) {
    //          default: {
    //            mapped_button = JoystickButtons::kA as usize;
    //          } break;
    //          case 1: {
    //            mapped_button = JoystickButtons::kB as usize;
    //          } break;
    //          case 2: {
    //            mapped_button = JoystickButtons::X as usize;
    //          } break;
    //          case 3: {
    //            mapped_button = JoystickButtons::Y as usize;
    //          } break;
    //          case 4: {
    //            mapped_button = JoystickButtons::LeftShoulder as usize;
    //          } break;
    //          case 5: {
    //            mapped_button = JoystickButtons::RightShoulder as usize;
    //          } break;
    //          case 6: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = kDualShockLeftTrigger;
    //            else
    //              mapped_button = JoystickButtons::Back as usize;
    //          } break;
    //          case 7: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = kDualShockRightTrigger;
    //            else
    //              mapped_button = JoystickButtons::Start as usize;
    //          } break;
    //          case 8: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::Back as usize;
    //            else
    //              mapped_button = JoystickButtons::LeftThumb as usize;
    //          } break;
    //          case 9: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::Start as usize;
    //            else
    //              mapped_button = JoystickButtons::RightThumb as usize;
    //          } break;
    //          case 10: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = kDualShockHome;
    //            else
    //              ;
    //          } break;
    //          case 11: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::LeftThumb as usize;
    //            else
    //              ;
    //          } break;
    //          case 12: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::RightThumb as usize;
    //            else
    //              ;
    //          } break;
    //          case 13: {
    //            mapped_button = JoystickButtons::DpadUp as usize;
    //          } break;
    //          case 14: {
    //            mapped_button = JoystickButtons::DpadDown as usize;
    //          } break;
    //          case 15: {
    //            mapped_button = JoystickButtons::DpadLeft as usize;
    //          } break;
    //          case 16: {
    //            mapped_button = JoystickButtons::DpadRight as usize;
    //          } break;
    //          }
    //          self.joystick[i].button[mapped_button].is_pressed = true;
    //          self.joystick[i].button[mapped_button].is_held = false;
    //          // printf("%s pressed button: %d\n", self.joystick[i].name,
    //          //        self.joystick[i].event.number);
    //        } else {
    //          i32 mapped_button = 0;
    //          switch (self.joystick[i].event.number) {
    //          default: {
    //            mapped_button = JoystickButtons::kA as usize;
    //          } break;
    //          case 1: {
    //            mapped_button = JoystickButtons::kB as usize;
    //          } break;
    //          case 2: {
    //            mapped_button = JoystickButtons::X as usize;
    //          } break;
    //          case 3: {
    //            mapped_button = JoystickButtons::Y as usize;
    //          } break;
    //          case 4: {
    //            mapped_button = JoystickButtons::LeftShoulder as usize;
    //          } break;
    //          case 5: {
    //            mapped_button = JoystickButtons::RightShoulder as usize;
    //          } break;
    //          case 6: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = kDualShockLeftTrigger;
    //            else
    //              mapped_button = JoystickButtons::Back as usize;
    //          } break;
    //          case 7: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = kDualShockRightTrigger;
    //            else
    //              mapped_button = JoystickButtons::Start as usize;
    //          } break;
    //          case 8: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::Back as usize;
    //            else
    //              mapped_button = JoystickButtons::LeftThumb as usize;
    //          } break;
    //          case 9: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::Start as usize;
    //            else
    //              mapped_button = JoystickButtons::RightThumb as usize;
    //          } break;
    //          case 10: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = kDualShockHome;
    //            else
    //              ;
    //          } break;
    //          case 11: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::LeftThumb as usize;
    //            else
    //              ;
    //          } break;
    //          case 12: {
    //            if (self.joystick[i].name[0] != 'X')
    //              mapped_button = JoystickButtons::RightThumb as usize;
    //            else
    //              ;
    //          } break;
    //          case 13: {
    //            mapped_button = JoystickButtons::DpadUp as usize;
    //          } break;
    //          case 14: {
    //            mapped_button = JoystickButtons::DpadDown as usize;
    //          } break;
    //          case 15: {
    //            mapped_button = JoystickButtons::DpadLeft as usize;
    //          } break;
    //          case 16: {
    //            mapped_button = JoystickButtons::DpadRight as usize;
    //          } break;
    //          }
    //          self.joystick[i].button[mapped_button].is_released = true;
    //          self.joystick[i].button[mapped_button].is_held = false;
    //          // printf("%s released button: %d\n", self.joystick[i].name,
    //          //        self.joystick[i].event.number);
    //        }

    //      } break;
    //      case JS_EVENT_AXIS: {
    //        if (self.joystick[i].event.number < 6) {
    //          self.joystick[i].axis[self.joystick[i].event.number] = self.joystick[i].event.value;
    //        } else {
    //          switch (self.joystick[i].event.number) {
    //          default: {
    //            if (self.joystick[i].event.value > 0) {
    //              self.joystick[i].button[JoystickButtons::DpadRight as usize].is_pressed = true;
    //              self.joystick[i].button[JoystickButtons::DpadRight as usize].is_held = false;
    //            } else if (self.joystick[i].event.value < 0) {
    //              self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_pressed = true;
    //              self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_held = false;
    //            } else {
    //              if (self.joystick[i].button[JoystickButtons::DpadRight as usize].is_held) {
    //                self.joystick[i].button[JoystickButtons::DpadRight as usize].is_released = true;
    //                self.joystick[i].button[JoystickButtons::DpadRight as usize].is_held = false;
    //              } else {
    //                self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_released = true;
    //                self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_held = false;
    //              }
    //            }
    //          } break;
    //          case 7: {
    //            if (self.joystick[i].event.value > 0) {
    //              self.joystick[i].button[JoystickButtons::DpadDown as usize].is_pressed = true;
    //              self.joystick[i].button[JoystickButtons::DpadDown as usize].is_held = false;
    //            } else if (self.joystick[i].event.value < 0) {
    //              self.joystick[i].button[JoystickButtons::DpadUp as usize].is_pressed = true;
    //              self.joystick[i].button[JoystickButtons::DpadUp as usize].is_held = false;
    //            } else {
    //              if (self.joystick[i].button[JoystickButtons::DpadDown as usize].is_held) {
    //                self.joystick[i].button[JoystickButtons::DpadDown as usize].is_released = true;
    //                self.joystick[i].button[JoystickButtons::DpadDown as usize].is_held = false;
    //              } else {
    //                self.joystick[i].button[JoystickButtons::DpadUp as usize].is_released = true;
    //                self.joystick[i].button[JoystickButtons::DpadUp as usize].is_held = false;
    //              }
    //            }
    //          } break;
    //          }
    //        }
    //        // printf("%s moved axis %d with value: %d \n", self.joystick[i].name,
    //        //        self.joystick[i].event.number, self.joystick[i].event.value);
    //      } break;
    //      default: {
    //        // JS_EVENT_INIT event
    //      } break;
    //      }
    //    }
    //    if (errno == EAGAIN) {
    //      // Queue is empty
    //    } else if (errno == ENODEV) {
    //      // printf("%s was disconnected on port %d\n", self.joystick[i].name, i);
    //      //   Device got disconnected
    //      close(self.joystick[i].fd);
    //      self.joystick[i].event = (struct js_event){0};
    //      self.joystick[i].fd = -1;
    //      memset(self.joystick[i].name, 0, sizeof(self.joystick[i].name));
    //      self.joystick[i].is_on = false;
    //      self.joystick[i].axes_n = 0;
    //      self.joystick[i].buttons_n = 0;
    //      for (i8 j = 0; j < 32; j++)
    //        self.joystick[i].button[j] = (struct Key){0};
    //      for (i8 j = 0; j < 16; j++)
    //        self.joystick[i].axis[j] = 0;
    //    } else if (errno == EBADF) {
    //      // Trying to read invalid FD
    //    }
    //  }

    // TODO: Maybe investigate Winmm for joystick input (only if it's better for
    // this use)
        for i in 0..4 {
            self.joystick[i].is_on = winapi::um::xinput::XInputGetState(
                i as u32,
                self.joystick[i].js_state.as_mut_ptr()
                ) == 0;
            if self.joystick[i].is_on {
                // Joystick is connected

                if !self.joystick[i].button[JoystickButtons::DpadUp as usize].is_held {
                    self.joystick[i].button[JoystickButtons::DpadUp as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_DPAD_UP) != 0;
                    self.joystick[i].button[JoystickButtons::DpadUp as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_DPAD_UP) == 0 {
                    self.joystick[i].button[JoystickButtons::DpadUp as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::DpadUp as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::DpadDown as usize].is_held {
                    self.joystick[i].button[JoystickButtons::DpadDown as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_DPAD_DOWN) != 0;
                self.joystick[i].button[JoystickButtons::DpadDown as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_DPAD_DOWN) == 0 {
                    self.joystick[i].button[JoystickButtons::DpadDown as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::DpadDown as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_held {
                    self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_DPAD_LEFT) != 0;
                self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_DPAD_LEFT) == 0 {
                    self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::DpadLeft as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::DpadRight as usize].is_held {
                    self.joystick[i].button[JoystickButtons::DpadRight as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_DPAD_RIGHT) != 0;
                self.joystick[i].button[JoystickButtons::DpadRight as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_DPAD_RIGHT) == 0 {
                    self.joystick[i].button[JoystickButtons::DpadRight as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::DpadRight as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::Start as usize].is_held {
                    self.joystick[i].button[JoystickButtons::Start as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_START) != 0;
                    self.joystick[i].button[JoystickButtons::Start as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_START) == 0 {
                    self.joystick[i].button[JoystickButtons::Start as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::Start as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::Back as usize].is_held {
                    self.joystick[i].button[JoystickButtons::Back as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_BACK) != 0;
                    self.joystick[i].button[JoystickButtons::Back as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_BACK) == 0 {
                    self.joystick[i].button[JoystickButtons::Back as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::Back as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::LeftThumb as usize].is_held {
                    self.joystick[i].button[JoystickButtons::LeftThumb as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_LEFT_THUMB) != 0;
                    self.joystick[i].button[JoystickButtons::LeftThumb as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_LEFT_THUMB) == 0 {
                    self.joystick[i].button[JoystickButtons::LeftThumb as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::LeftThumb as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::RightThumb as usize].is_held {
                    self.joystick[i].button[JoystickButtons::RightThumb as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_RIGHT_THUMB) != 0;
                    self.joystick[i].button[JoystickButtons::RightThumb as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_RIGHT_THUMB) == 0 {
                    self.joystick[i].button[JoystickButtons::RightThumb as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::RightThumb as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::LeftShoulder as usize].is_held {
                    self.joystick[i].button[JoystickButtons::LeftShoulder as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                        winapi::um::xinput::XINPUT_GAMEPAD_LEFT_SHOULDER) != 0;
                    self.joystick[i].button[JoystickButtons::LeftShoulder as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_LEFT_SHOULDER) == 0 {
                    self.joystick[i].button[JoystickButtons::LeftShoulder as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::LeftShoulder as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::RightShoulder as usize].is_held {
                    self.joystick[i].button[JoystickButtons::RightShoulder as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                        winapi::um::xinput::XINPUT_GAMEPAD_RIGHT_SHOULDER) != 0;
                    self.joystick[i].button[JoystickButtons::RightShoulder as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_RIGHT_SHOULDER) == 0 {
                    self.joystick[i].button[JoystickButtons::RightShoulder as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::RightShoulder as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::A as usize].is_held {
                    self.joystick[i].button[JoystickButtons::A as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_A) != 0;
                    self.joystick[i].button[JoystickButtons::A as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_A) == 0 {
                    self.joystick[i].button[JoystickButtons::A as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::A as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::B as usize].is_held {
                    self.joystick[i].button[JoystickButtons::B as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_B) != 0;
                    self.joystick[i].button[JoystickButtons::B as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_B) == 0 {
                    self.joystick[i].button[JoystickButtons::B as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::B as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::X as usize].is_held {
                    self.joystick[i].button[JoystickButtons::X as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_X) != 0;
                    self.joystick[i].button[JoystickButtons::X as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_X) == 0 {
                    self.joystick[i].button[JoystickButtons::X as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::X as usize].is_held = false;
                }
                if !self.joystick[i].button[JoystickButtons::Y as usize].is_held {
                    self.joystick[i].button[JoystickButtons::Y as usize].is_pressed =
                        ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                         winapi::um::xinput::XINPUT_GAMEPAD_Y) != 0;
                    self.joystick[i].button[JoystickButtons::Y as usize].is_held = false;
                } else if ((*self.joystick[i].js_state.as_ptr()).Gamepad.wButtons &
                    winapi::um::xinput::XINPUT_GAMEPAD_Y) == 0 {
                    self.joystick[i].button[JoystickButtons::Y as usize].is_released = true;
                    self.joystick[i].button[JoystickButtons::Y as usize].is_held = false;
                }

                self.joystick[i].axis[JoystickAxes::LeftTrigger as usize] =
                    (*self.joystick[i].js_state.as_ptr()).Gamepad.bLeftTrigger as i32;
                self.joystick[i].axis[JoystickAxes::RightTrigger as usize] =
                    (*self.joystick[i].js_state.as_ptr()).Gamepad.bRightTrigger as i32;

                self.joystick[i].axis[JoystickAxes::LeftThumbX as usize] =
                    (*self.joystick[i].js_state.as_ptr()).Gamepad.sThumbLX as i32;
                self.joystick[i].axis[JoystickAxes::LeftThumbY as usize] =
                    (*self.joystick[i].js_state.as_ptr()).Gamepad.sThumbLY as i32;
                self.joystick[i].axis[JoystickAxes::RightThumbX as usize] =
                    (*self.joystick[i].js_state.as_ptr()).Gamepad.sThumbRX as i32;
                self.joystick[i].axis[JoystickAxes::RightThumbY as usize] =
                    (*self.joystick[i].js_state.as_ptr()).Gamepad.sThumbRY as i32;
            } else {
            // Joystick is not connected
            }
        }
    }


    pub fn d(&self, x : i32, y : i32, color : u32){
        if x < 0 || y < 0 || x >= self.width || y >= self.height { return }
        //#ifndef _WIN32
        //  // On Xlib the y-axis is inverted (compared to windows)
        //  *((u32 *)W.bitmap_memory + (W.height - 1 - y) * W.width + x) = color;
        //#else
        //  *((u32 *)W.bitmap_memory + y * W.width + x) = color;
        //#endif
        let offset : isize = (y * self.width + x) as isize;
        unsafe { *(self.bitmap_memory as *mut u32).offset(offset) = color; }
    }

    pub fn c(&self, color : u32){
        //  for (i32 i = 0; i < W.width; ++i)
        //    for (i32 j = 0; j < W.height; ++j)
        //#ifndef _WIN32
        //      // On X11 the y-axis is inverted (compared to windows)
        //      *((u32 *)W.bitmap_memory + (W.height - 1 - j) * W.width + i) = color;
        //#else
        //      *((u32 *)W.bitmap_memory + j * W.width + i) = color;
        //#endif
        let win_sz : i32 = self.width * self.height;
        for i in 0..win_sz{
            unsafe { *(self.bitmap_memory as *mut u32).offset(i as isize) = color; }
        }
    }
}

pub struct Mouse{
    pub x : i16,
    pub y : i16,
    pub left_pressed : bool,
    pub middle_pressed : bool,
    pub right_pressed : bool,
    pub left_held : bool,
    pub middle_held : bool,
    pub right_held : bool,
    pub left_released : bool,
    pub middle_released : bool,
    pub right_released : bool,
    pub is_focused : bool,
}

pub struct Key{
    pub is_pressed : bool,
    pub is_held : bool,
    pub is_released : bool,
}

pub enum MouseButtons {
  MouseLeft = 1,
  MouseMiddle,
  MouseRight,
  MouseScrollUp,
  MouseScrollDown
}

pub enum OtherKeys {
  Escape = 128,
  Enter,
  Tab,
  Backspace,
  Insert,
  Delete,
  Right,
  Left,
  Down,
  Up,
  PageUp,
  PageDown,
  Home,
  End,
  Capslock,
  ScrollLock,
  NumLock,
  PrintScreen,
  PauseBreak,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  Numpad0,
  Numpad1,
  Numpad2,
  Numpad3,
  Numpad4,
  Numpad5,
  Numpad6,
  Numpad7,
  Numpad8,
  Numpad9,
  Decimal,
  Divide,
  Multiply,
  Subtract,
  Add,
  LeftShift,
  LeftCtrl,
  LeftAlt,
  LeftSuper,
  RightShift,
  RightCtrl,
  RightAlt,
  RightSuper
}

pub enum JoystickButtons {
  DpadUp = 0,
  DpadDown,
  DpadLeft,
  DpadRight,
  Start,
  Back,
  LeftThumb,
  RightThumb,
  LeftShoulder,
  RightShoulder,
  A,
  B,
  X,
  Y,
  DualShockLeftTrigger,
  DualShockRightTrigger,
  DualShockHome
}

pub enum JoystickAxes {
  LeftThumbX = 0,
  LeftThumbY,
  LeftTrigger,
  RightThumbX,
  RightThumbY,
  RightTrigger
}

// TODO: maybe joysticks should also have repeating (like keys)?
pub struct Joystick {
//#ifndef _WIN32
//  struct js_event event;
//  i32 fd;
//  char path[16];
//#else
  pub js_state : MaybeUninit<winapi::um::xinput::XINPUT_STATE>,
  // f64
  // u8 timer;
  pub name : &'static str,
  pub is_on : bool,
  pub axes_n : i32,
  pub buttons_n : i32,
  pub button : [Key; 32],
  pub axis: [i32; 16],
}
//#ifndef _WIN32
//    {{0}, -1, "/dev/input/js0", {0}, false, 0, 0, {0}, {0}},
//    {{0}, -1, "/dev/input/js1", {0}, false, 0, 0, {0}, {0}},
//    {{0}, -1, "/dev/input/js2", {0}, false, 0, 0, {0}, {0}},
//    {{0}, -1, "/dev/input/js3", {0}, false, 0, 0, {0}, {0}}
//#else
//    // To Windows every controller is Xbox Controller
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}},
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}},
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}},
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}}

pub struct Sprite {
    pub x : i32, pub y : i32,
    pub width : i32, pub height : i32, pub channels : i32,
    pub data : Vec<Vec<u8>>
}

pub struct Window{
    pub title : String,
    pub audio_engine: MaybeUninit<miniaudio::ma_engine>,
    pub musics : HashMap<&'static str, Music>,
    pub audio_decoders : HashMap<&'static str, MaybeUninit<miniaudio::ma_decoder>>,
    pub decoder_config : MaybeUninit<miniaudio::ma_decoder_config>,
    pub internal_font_5x6 : Sprite,
    pub internal_font_5x5 : Sprite,
    pub width : i32,
    pub height : i32,
    pub previous_width : i32,
    pub previous_height : i32,
    pub offset_x : i32,
    pub offset_y : i32,
    pub current_pos_x : i32,
    pub current_pos_y : i32,
    pub is_running : bool,
    pub is_fullscreen: bool,
    pub is_focused : bool,
    pub clock : Clock,
    pub mouse : Mouse,
    pub key : [Key; 256],
    pub joystick : [Joystick; 4],
    pub bitmap_memory : *mut winapi::ctypes::c_void,
    pub bitmap_info : winapi::um::wingdi::BITMAPINFO,
    pub bitmap_device_context : winapi::shared::windef::HDC,
    pub handle : winapi::shared::windef::HWND,
    pub instance: winapi::shared::minwindef::HINSTANCE,
}

macro_rules! array {
    (@accum (0, $($_es:expr),*) -> ($($body:tt)*))
        => {array!(@as_expr [$($body)*])};
    (@accum (1, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (2, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (3, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (4, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (5, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (6, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (7, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)* $($es,)*))};
    (@accum (8, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (16, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (8, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (32, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (16, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (64, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (32, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (128, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (64, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (256, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (128, $($es,)* $($es),*) -> ($($body)*))};

    (@as_expr $e:expr) => {$e};

    [$e:expr; $n:tt] => { array!(@accum ($n, $e) -> ()) };
}

impl Default for Window {
    fn default () -> Window {
        Window{
            title : String::new(),
            audio_engine : MaybeUninit::uninit(),
            musics : HashMap::new(),
            audio_decoders : HashMap::new(),
            decoder_config : MaybeUninit::new(unsafe{ miniaudio::ma_decoder_config_init_default() }),
            // Always load these fonts
            internal_font_5x6 : alloc_sprite(include_bytes!("../font5x6.png")),
            internal_font_5x5 : alloc_sprite(include_bytes!("../font5x5.png")),
            width : 0,
            height : 0,
            previous_width : 0,
            previous_height : 0,
            offset_x : 0,
            offset_y : 0,
            current_pos_x : 0,
            current_pos_y : 0,
            is_running : true,
            is_fullscreen : false,
            is_focused : true,
            clock : Clock::default(),
            mouse : Mouse{
                x : 0,
                y : 0,
                left_pressed : false,
                middle_pressed : false,
                right_pressed : false,
                left_held : false,
                middle_held : false,
                right_held : false,
                left_released : false,
                middle_released : false,
                right_released : false,
                is_focused : false,
            },
            key : array![Key{is_pressed : false, is_held : false, is_released : false}; 256],
//pub struct Joystick {
//  pub js_state : winapi::um::xinput::XINPUT_STATE,
//  name : String,
//  is_on : bool,
//  axes_n : i32,
//  buttons_n : i32,
//  button : [Key; 32],
//  axis: [i32; 16],
//}
//    // To Windows every controller is Xbox Controller
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}},
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}},
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}},
//    {{0}, {"Xbox Controller"}, false, 6, 14, {0}, {0}}

            joystick : array![
                Joystick{
                    js_state : MaybeUninit::new(winapi::um::xinput::XINPUT_STATE{
                        dwPacketNumber : 0,
                        Gamepad : winapi::um::xinput::XINPUT_GAMEPAD{
                            wButtons : 0,
                            bLeftTrigger : 0,
                            bRightTrigger : 0,
                            sThumbLX : 0,
                            sThumbLY : 0,
                            sThumbRX : 0,
                            sThumbRY : 0
                        }
                    }),
                    name : "Xbox Controller",
                    is_on : false,
                    axes_n : 6,
                    buttons_n : 14,
                    button : array![Key{is_pressed : false, is_held : false, is_released : false}; 32],
                    axis : array![0; 16]
                }
                ; 4],
            bitmap_memory : 0 as *mut winapi::ctypes::c_void,
            bitmap_info : winapi::um::wingdi::BITMAPINFO {
                bmiHeader : winapi::um::wingdi::BITMAPINFOHEADER {
                    biSize : std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
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

pub struct Clock{
    pub last_second_elapsed_time : f64,
    pub last_frame_elapsed_time : f64,
    pub total_elapsed_time : f64,
    pub frame : u64,
    pub frames_last_second : i32,
    pub time1 : time::Instant,
    pub time2 : time::Instant,
}

impl Clock{
    fn tick(&mut self){
        self.time2 = time::Instant::now();
        self.last_frame_elapsed_time = self.time2.duration_since(self.time1).as_secs_f64();
        self.time1 = self.time2;
        self.last_second_elapsed_time = self.last_second_elapsed_time + self.last_frame_elapsed_time;
        self.total_elapsed_time = self.total_elapsed_time + self.last_frame_elapsed_time;
        self.frame = self.frame + 1;
        if self.last_second_elapsed_time > 1.{
            self.frames_last_second = self.frame as i32;
            self.last_second_elapsed_time = 0.;
            self.frame = 0;
        }
    }  
}

impl Default for Clock{
    fn default() -> Clock{
        Clock{
            last_second_elapsed_time : 0.0,
            last_frame_elapsed_time : 0.0,
            total_elapsed_time : 0.0,
            frame : 0,
            frames_last_second : 0,
            time1 : time::Instant::now(),
            time2 : time::Instant::now(),
        }
    }
}

unsafe fn update_key_state(window_handle : winapi::shared::windef::HWND , key : i32, bitfield : u32) {
    let window : *mut Window = winapi::um::winuser::GetWindowLongPtrW(window_handle, 0) as *mut Window;    
    // TODO: check to make sure that linux is 100% consistent with windows
    //#ifndef _WIN32
    //  bool is_down = bitfield;
    //
    //  switch (key) {
    //  case 0xFF1B:
    //    mapped_key = kEscape;
    //    break;
    //  case 0xFF0D:
    //  case 0xFF8D:
    //    mapped_key = kEnter;
    //    break;
    //  case 0xFF09:
    //    mapped_key = kTab;
    //    break;
    //  case 0xFF08:
    //    mapped_key = kBackspace;
    //    break;
    //  case 0xFF63:
    //    mapped_key = kInsert;
    //    break;
    //  case 0xFFFF:
    //  case 0xFF9F:
    //    mapped_key = kDelete;
    //    break;
    //  case 0xFF53:
    //    mapped_key = kRight;
    //    break;
    //  case 0xFF51:
    //    mapped_key = kLeft;
    //    break;
    //  case 0xFF54:
    //    mapped_key = kDown;
    //    break;
    //  case 0xFF52:
    //    mapped_key = kUp;
    //    break;
    //  case 0xFF55:
    //    mapped_key = kPageUp;
    //    break;
    //  case 0xFF56:
    //    mapped_key = kPageDown;
    //    break;
    //  // case 0xFF:
    //  //   mapped_key = kHome;
    //  //   break;
    //  case 0xFF57:
    //    mapped_key = kEnd;
    //    break;
    //  case 0xFFE5:
    //    mapped_key = kCapslock;
    //    break;
    //  case 0xFF14:
    //    mapped_key = kScrollLock;
    //    break;
    //  case 0xFF7F:
    //    mapped_key = kNumLock;
    //    break;
    //  // case 0xFF:
    //  //   mapped_key = kPrintScreen;
    //  //   break;
    //  case 0xFF13:
    //    mapped_key = kPauseBreak;
    //    break;
    //  case 0xFFBE:
    //    mapped_key = kF1;
    //    break;
    //  case 0xFFBF:
    //    mapped_key = kF2;
    //    break;
    //  case 0xFFC0:
    //    mapped_key = kF3;
    //    break;
    //  case 0xFFC1:
    //    mapped_key = kF4;
    //    break;
    //  case 0xFFC2:
    //    mapped_key = kF5;
    //    break;
    //  case 0xFFC3:
    //    mapped_key = kF6;
    //    break;
    //  case 0xFFC4:
    //    mapped_key = kF7;
    //    break;
    //  case 0xFFC5:
    //    mapped_key = kF8;
    //    break;
    //  case 0xFFC6:
    //    mapped_key = kF9;
    //    break;
    //  case 0xFFC7:
    //    mapped_key = kF10;
    //    break;
    //  case 0xFFC8:
    //    mapped_key = kF11;
    //    break;
    //  case 0xFFC9:
    //    mapped_key = kF12;
    //    break;
    //  case 0xFF9E:
    //    mapped_key = kNumpad0;
    //    break;
    //  case 0xFF9C:
    //    mapped_key = kNumpad1;
    //    break;
    //  case 0xFF99:
    //    mapped_key = kNumpad2;
    //    break;
    //  case 0xFF9B:
    //    mapped_key = kNumpad3;
    //    break;
    //  case 0xFF96:
    //    mapped_key = kNumpad4;
    //    break;
    //  case 0xFF9D:
    //    mapped_key = kNumpad5;
    //    break;
    //  case 0xFF98:
    //    mapped_key = kNumpad6;
    //    break;
    //  case 0xFF95:
    //    mapped_key = kNumpad7;
    //    break;
    //  case 0xFF97:
    //    mapped_key = kNumpad8;
    //    break;
    //  case 0xFF9A:
    //    mapped_key = kNumpad9;
    //    break;
    //  // case 0xFF:
    //  //   mapped_key = kDecimal;
    //  //   break;
    //  case 0xFFAF:
    //    mapped_key = kDivide;
    //    break;
    //  case 0xFFAA:
    //    mapped_key = kMultiply;
    //    break;
    //  case 0xFFAD:
    //    mapped_key = kSubtract;
    //    break;
    //  case 0xFFAB:
    //    mapped_key = kAdd;
    //    break;
    //  case 0xFFE1:
    //    mapped_key = kLeftShift;
    //    break;
    //  case 0xFFE3:
    //    mapped_key = kLeftCtrl;
    //    break;
    //  case 0xFFE9:
    //    mapped_key = kLeftAlt;
    //    break;
    //  case 0xFFEB:
    //    mapped_key = kLeftSuper;
    //    break;
    //  case 0xFFE2:
    //    mapped_key = kRightShift;
    //    break;
    //  case 0xFFE4:
    //    mapped_key = kRightCtrl;
    //    break;
    //  case 0xFF03:
    //    mapped_key = kRightAlt;
    //    break;
    //  case 0xFF67: // NOTE: I don't think that this key is Super but it works for
    //               // now
    //    mapped_key = kRightSuper;
    //    break;
    //  }
    //  if (mapped_key >= 'a' && mapped_key <= 'z') {
    //    Key[mapped_key - 32].is_pressed = is_down;
    //    Key[mapped_key - 32].is_held = false;
    //    Key[mapped_key - 32].is_released = !is_down;
    //  }
    //  Key[mapped_key].is_pressed = is_down;
    //  Key[mapped_key].is_held = false;
    //  Key[mapped_key].is_released = !is_down;
    //#else
    let was_down : bool = (bitfield >> 30 & 1) == 1;
    let is_down  : bool = (bitfield >> 31 ^ 1) == 1;
    let mapped_key = match key {
        winapi::um::winuser::VK_ESCAPE   => OtherKeys::Escape      as i32,
        winapi::um::winuser::VK_RETURN   => OtherKeys::Enter       as i32,
        winapi::um::winuser::VK_TAB      => OtherKeys::Tab         as i32,
        winapi::um::winuser::VK_BACK     => OtherKeys::Backspace   as i32,
        winapi::um::winuser::VK_INSERT   => OtherKeys::Insert      as i32,
        winapi::um::winuser::VK_DELETE   => OtherKeys::Delete      as i32,
        winapi::um::winuser::VK_RIGHT    => OtherKeys::Right       as i32,
        winapi::um::winuser::VK_LEFT     => OtherKeys::Left        as i32,
        winapi::um::winuser::VK_DOWN     => OtherKeys::Down        as i32,
        winapi::um::winuser::VK_UP       => OtherKeys::Up          as i32,
        winapi::um::winuser::VK_PRIOR    => OtherKeys::PageUp      as i32,
        winapi::um::winuser::VK_NEXT     => OtherKeys::PageDown    as i32,
        winapi::um::winuser::VK_HOME     => OtherKeys::Home        as i32,
        winapi::um::winuser::VK_END      => OtherKeys::End         as i32,
        winapi::um::winuser::VK_CAPITAL  => OtherKeys::Capslock    as i32,
        winapi::um::winuser::VK_SCROLL   => OtherKeys::ScrollLock  as i32,
        winapi::um::winuser::VK_NUMLOCK  => OtherKeys::NumLock     as i32,
        winapi::um::winuser::VK_SNAPSHOT => OtherKeys::PrintScreen as i32,
        winapi::um::winuser::VK_PAUSE    => OtherKeys::PauseBreak  as i32,
        winapi::um::winuser::VK_F1       => OtherKeys::F1          as i32,
        winapi::um::winuser::VK_F2       => OtherKeys::F2          as i32,
        winapi::um::winuser::VK_F3       => OtherKeys::F3          as i32,
        winapi::um::winuser::VK_F4       => OtherKeys::F4          as i32,
        winapi::um::winuser::VK_F5       => OtherKeys::F5          as i32,
        winapi::um::winuser::VK_F6       => OtherKeys::F6          as i32,
        winapi::um::winuser::VK_F7       => OtherKeys::F7          as i32,
        winapi::um::winuser::VK_F8       => OtherKeys::F8          as i32,
        winapi::um::winuser::VK_F9       => OtherKeys::F9          as i32,
        winapi::um::winuser::VK_F10      => OtherKeys::F10         as i32,
        winapi::um::winuser::VK_F11      => OtherKeys::F11         as i32,
        winapi::um::winuser::VK_F12      => OtherKeys::F12         as i32,
        winapi::um::winuser::VK_NUMPAD0  => OtherKeys::Numpad0     as i32,
        winapi::um::winuser::VK_NUMPAD1  => OtherKeys::Numpad1     as i32,
        winapi::um::winuser::VK_NUMPAD2  => OtherKeys::Numpad2     as i32,
        winapi::um::winuser::VK_NUMPAD3  => OtherKeys::Numpad3     as i32,
        winapi::um::winuser::VK_NUMPAD4  => OtherKeys::Numpad4     as i32,
        winapi::um::winuser::VK_NUMPAD5  => OtherKeys::Numpad5     as i32,
        winapi::um::winuser::VK_NUMPAD6  => OtherKeys::Numpad6     as i32,
        winapi::um::winuser::VK_NUMPAD7  => OtherKeys::Numpad7     as i32,
        winapi::um::winuser::VK_NUMPAD8  => OtherKeys::Numpad8     as i32,
        winapi::um::winuser::VK_NUMPAD9  => OtherKeys::Numpad9     as i32,
        winapi::um::winuser::VK_DECIMAL  => OtherKeys::Decimal     as i32,
        winapi::um::winuser::VK_DIVIDE   => OtherKeys::Divide      as i32,
        winapi::um::winuser::VK_MULTIPLY => OtherKeys::Multiply    as i32,
        winapi::um::winuser::VK_SUBTRACT => OtherKeys::Subtract    as i32,
        winapi::um::winuser::VK_ADD      => OtherKeys::Add         as i32,
        // TODO: fix special VK_KEYS with right variants (or leave as is)
        winapi::um::winuser::VK_SHIFT |
        winapi::um::winuser::VK_LSHIFT   => OtherKeys::LeftShift   as i32,
        winapi::um::winuser::VK_CONTROL |
        winapi::um::winuser::VK_LCONTROL => OtherKeys::LeftCtrl    as i32,
        winapi::um::winuser::VK_MENU |
        winapi::um::winuser::VK_LMENU    => OtherKeys::LeftAlt     as i32,
        winapi::um::winuser::VK_LWIN     => OtherKeys::LeftSuper   as i32,
        winapi::um::winuser::VK_RSHIFT   => OtherKeys::RightShift  as i32,
        winapi::um::winuser::VK_RCONTROL => OtherKeys::RightCtrl   as i32,
        winapi::um::winuser::VK_RMENU    => OtherKeys::RightAlt    as i32,
        winapi::um::winuser::VK_RWIN     => OtherKeys::RightSuper  as i32,
        _ => key,
    };
    if (mapped_key as u8 as char) >= 'A' && (mapped_key as u8 as char) <= 'Z' {
        (*window).key[mapped_key as usize + 32].is_pressed = is_down;
        (*window).key[mapped_key as usize + 32].is_held = false;
        (*window).key[mapped_key as usize + 32].is_released = (was_down) & (!is_down);
    }
    (*window).key[mapped_key as usize].is_pressed = is_down;
    (*window).key[mapped_key as usize].is_held = false;
    (*window).key[mapped_key as usize].is_released = (was_down) & (!is_down);
    //#endif
}

pub fn alloc_sprite(data : &[u8]) -> Sprite {
    let mut width = MaybeUninit::new(0);
    let mut height = MaybeUninit::new(0);
    let mut channels = MaybeUninit::new(0);

    unsafe{
        let reverse_rows = 
            stb_image::stbi_load_from_memory(
                data.as_ptr() as *const u8,
                data.len() as i32,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                channels.as_mut_ptr(),
                4);
        // Reverse row order (from "left to right, top to bottom" to "left to right,
        // bottom to top")
        let mut ordered_rows = vec![vec![0; (*width.as_ptr() * 4) as usize]; *height.as_ptr() as usize];
        for i in 0..(*height.as_ptr()) {
            for j in 0..(*width.as_ptr() * 4) {
                ordered_rows[i as usize][j as usize] = *(reverse_rows.offset(
                        ((*height.as_ptr() - 1 - i) * 4 * (*width.as_ptr()) + j) as isize
                    ));
            }
        }

        return Sprite{
                x : 0, y : 0,
                width : *width.as_ptr(), height : *height.as_ptr(), channels : *channels.as_ptr(),
                data : ordered_rows
            };
    }
}

pub fn free_sprite(spr : &mut Sprite) {
    for i in 0..spr.data.len(){
        spr.data[i].clear();
    }
    spr.data.clear();
}

