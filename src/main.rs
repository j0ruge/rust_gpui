use gpui::{
    App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};
use gpui_component::{Root, Theme, ThemeMode};
use gpui_component_assets::Assets;
use rust_gpui::ui::TaskScreen;

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        gpui_component::init(cx);
        Theme::change(ThemeMode::Dark, None, cx);
        let bounds = Bounds::centered(None, size(px(900.), px(640.)), cx);
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(640.), px(480.))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Tarefas".into()),
                    ..Default::default()
                }),
                app_id: Some("br.jrc.rust-gpui".into()),
                ..Default::default()
            },
            |window, cx| {
                let screen = cx.new(|_| TaskScreen);
                cx.new(|cx| Root::new(screen, window, cx))
            },
        );
        if let Err(error) = result {
            eprintln!("Não foi possível abrir a janela: {error}");
            cx.quit();
            return;
        }
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                // Sair em outro ciclo do executor evita reentrar no backend X11.
                cx.spawn(async |cx| {
                    let _ = cx.update(|cx| cx.quit());
                })
                .detach();
            }
        })
        .detach();
        cx.activate(true);
    });
}
