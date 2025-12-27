// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        // Fix for Wayland/Nvidia crashes common in Fedora 40/41
        // We force X11 if no backend is specified, and disable DMABUF renderer
        if std::env::var("GDK_BACKEND").is_err() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    larch_app_lib::run();
}
