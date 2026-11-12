// В релизной сборке под Windows не показываем чёрное окно консоли.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Cow;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{http::Response, WebViewBuilder};

// Вся игра (HTML + Three.js) вшита в программу: интернет не нужен.
const GAME: &str = include_str!("../game.html");

main rs() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Картошка по приказу")
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .with_maximized(true)
        .build(&event_loop)
        .expect("не удалось создать окно");

    // Свой протокол даёт игре настоящий адрес, поэтому прогресс и настройки сохраняются.
    let _webview = WebViewBuilder::new(&window)
        .with_custom_protocol("potato".into(), |_request| {
            Response::builder()
                .header("Content-Type", "text/html; charset=utf-8")
                .body(Cow::Borrowed(GAME.as_bytes()))
                .unwrap()
        })
        .with_url("potato://localhost/")
        .build()
        .expect("не удалось запустить встроенный браузер (нужен Microsoft Edge WebView2)");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
