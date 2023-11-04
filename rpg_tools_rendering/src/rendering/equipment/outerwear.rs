use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::pants::interpolate_pants_y;
use crate::rendering::equipment::part::button::render_buttons;
use crate::rendering::equipment::part::neckline::get_neckline_bottom_y;
use crate::rendering::equipment::part::sleeve::render_sleeves;
use crate::rendering::equipment::shirt::create_shirt;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::outerwear::{
    ClosingOption, Coat, Outerwear, OuterwearLength,
};

pub fn render_outerwear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    outerwear: &Outerwear,
    from_front: bool,
) {
    match outerwear {
        Outerwear::None => {}
        Outerwear::Coat { style } => render_coat(renderer, config, aabb, body, style, from_front),
    }
}

fn render_coat(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    coat: &Coat,
    from_front: bool,
) {
    render_sleeves(renderer, config, aabb, body, coat.sleeve, coat.color);
    render_torso(renderer, config, aabb, body, coat, from_front);

    if from_front {
        render_closing(renderer, config, aabb, body, coat);
    }
}

fn render_torso(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    coat: &Coat,
    from_front: bool,
) {
    let options = config.get_options(coat.color);
    let mut builder = create_shirt(config, aabb, body, coat.neckline, from_front, 0.1);

    builder.reverse();

    let (pants_width, _inner_width) = config.pants.get_widths(&config.body, body);
    let pants_width = pants_width + 0.03;
    let hip_width =
        config.pants.get_hip_width(&config.body, body) * config.body.get_torso_width(body);
    let width = hip_width.max(pants_width);

    let y = get_bottom_y(config, body, coat);

    builder.add_mirrored_points(aabb, width, y, true);
    builder.add_point(aabb.get_point(0.5, y + 0.01), false);

    let polygon = builder.build();
    renderer.render_rounded_polygon(&polygon, &options);
}

fn render_closing(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    coat: &Coat,
) {
    let top_y = config
        .body
        .from_torso_to_body(get_neckline_bottom_y(&config.shirt, coat.neckline));
    let bottom_y = get_bottom_y(config, body, coat);

    match coat.closing {
        ClosingOption::None => {}
        ClosingOption::SingleBreasted { buttons } => {
            render_buttons(renderer, config, aabb, buttons, top_y, bottom_y, 0.5)
        }
        ClosingOption::DoubleBreasted { buttons } => {
            let offset = 0.05;
            render_buttons(
                renderer,
                config,
                aabb,
                buttons,
                top_y,
                bottom_y,
                0.5 - offset,
            );
            render_buttons(
                renderer,
                config,
                aabb,
                buttons,
                top_y,
                bottom_y,
                0.5 + offset,
            );
        }
        ClosingOption::Zipper { color } => {
            let option = config.line_with_color(color, 2.0);
            let top = aabb.get_point(0.5, top_y);
            let bottom = aabb.get_point(0.5, bottom_y);
            let line = Line2d::new(vec![top, bottom]);

            renderer.render_line(&line, &option);
        }
    }
}

fn get_bottom_y(config: &RenderConfig, body: &Body, coat: &Coat) -> f32 {
    let y_factor = match coat.length {
        OuterwearLength::Hip => config.pants.height_shorts,
        OuterwearLength::Knee => config.pants.height_bermuda,
        OuterwearLength::Ankle => config.pants.get_bottom_y(&config.body, body),
    };
    interpolate_pants_y(config, body, y_factor)
}
