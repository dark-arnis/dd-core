use conrod;
use conrod::glium;
use conrod::backend::glium::glium::{Surface};

use conrod::{Borderable};
use conrod::widget::*;

// use vst::plugin::VSTPlugin;

use vst2::plugin::HostCallback;

// extern crate rand;
// use self::rand::Rng;

#[derive(Debug)]
pub enum ConrodWindowError {
    GetWindowFail,
    GetInnerSizeFail,
    LoadRendererFail,
}

pub struct ConrodWindow {
    pub ui: conrod::Ui,
    pub display: glium::Display,
    pub image_map: conrod::image::Map<glium::texture::Texture2d>,
    pub ids: Ids,
    pub renderer: conrod::backend::glium::Renderer,
}

impl ConrodWindow {
    pub fn new(window: glium::Display) -> Result<Self, ConrodWindowError> {
        let (width, height) = try!(window.get_window()
            .ok_or(ConrodWindowError::GetWindowFail)
            .and_then({|window|
                window.get_inner_size().ok_or(ConrodWindowError::GetInnerSizeFail)
            }));
        
        info!("size : {}x{}", width, height);

        info!("framebuffer: {:?}", window.get_framebuffer_dimensions());

        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = match conrod::backend::glium::Renderer::new(&window) {
            Ok(r) => r,
            Err(e) => { error!("Error creating Renderer: {:?}", e); return Err(ConrodWindowError::LoadRendererFail) },
        };

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());
        
        Ok(ConrodWindow{ui: ui, display: window, image_map: image_map, renderer: renderer, ids: ids})
    }

    pub fn draw(&mut self, host: &mut HostCallback) {

        for event in self.display.poll_events() {
			// Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &self.display) {
                info!("-- event: {:?}", event);
                self.ui.handle_event(event);
            }
        };

        set_widgets(self.ui.set_widgets(), &mut self.ids, host);

        let mut target = self.display.draw();

        // self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();


        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
        }

        target.finish().unwrap();


        // Collect all pending events.
        // let mut events = Vec::new();
        // events.extend(self.display.poll_events());

        // for event in events {
        //     info!("-- event {:?}", event);
        //     match event {
        //         "Closed" => { info!(" closed."); },
        //         _ => ()
        //     }
        // }
    }
}

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, host: &mut HostCallback) {
    use conrod::{color, widget, Labelable, Colorable, Sizeable, Widget};

    Canvas::new()
        .color(color::BLUE)
        .border(0.5)
        .w_h(110.0, 150.0)
        .set(ids.body, ui);

    // let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::RED);

    let button = widget::Button::new().label("clickkk").color(color::RED).w_h(60.0, 30.0);
    for _click in button.floating(true).set(ids.button, ui) {
        info!("Bing!");
    }
}

widget_ids! {
    pub struct Ids {
        body,
        button,
    }
}