use gpui::*;

pub fn window_options(app_width: i32, app_height: i32, cx: &AppContext) -> WindowOptions {
    let display_id_maybe = cx.displays().last().map(|d| d.id());
    let bounds_maybe = cx.displays().last().map(|d| d.bounds());
    let bounds = bounds_maybe.unwrap_or(Bounds {
        origin: Point::new(DevicePixels::from(0), DevicePixels::from(0)),
        size: Size {
            width: DevicePixels::from(1920),
            height: DevicePixels::from(1080),
        },
    });

    let mut options = WindowOptions::default();
    let center = bounds.center();

    options.focus = true;
    options.display_id = display_id_maybe;
    let width = DevicePixels::from(app_width);
    let height = DevicePixels::from(app_height);
    let x: DevicePixels = center.x - width / 2;
    let y: DevicePixels = center.y - height / 2;

    let bounds: Bounds<DevicePixels> = Bounds::new(Point { x, y }, Size { width, height });
    options.bounds = Some(bounds);
    options.titlebar = Some(TitlebarOptions::default());
    options.is_movable = true;
    options.kind = WindowKind::PopUp;
    options
}
