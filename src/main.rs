use std::{rc::Rc, cell::RefCell};

use gtk::{prelude::*, Adjustment, Application, ApplicationWindow, Orientation};
use rodio::{Sink, OutputStream, source::SineWave};

const DEFAULT_FREQUENCY: f32 = 400.0;

fn main() {
    // audio stuff
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Rc::new(RefCell::new(Sink::try_new(&stream_handle).unwrap()));
    sink.try_borrow().unwrap().append(SineWave::new(DEFAULT_FREQUENCY));

    // graphics stuff
    let application = Application::builder()
        .application_id("com.bugger2.waves")
        .build();

    application.connect_startup(move |app| {
        let window = ApplicationWindow::builder()
            .default_width(900)
            .default_height(900)
            .resizable(true)
            .title("Waves")
            .application(app)
            .build();

        // container for the left/right slider and amplitude/graph/frequency view
        let container = gtk::Grid::builder()
            .column_homogeneous(true)
            .column_spacing(10)
            .build();

        // LEFT/RIGHT SLIDER
        const LEFT_RIGHT_WIDTH: i32 = 2;
        const LEFT_RIGHT_HEIGHT: i32 = 2;
        let left_right_slider = gtk::Scale::builder()
            .orientation(Orientation::Horizontal)
            .adjustment(&Adjustment::builder()
                        .lower(0.0)
                        .upper(100.0)
                        .step_increment(0.0)
                        .page_size(0.0)
                        .page_increment(0.0)
                        .value(50.0)
                        .build())
            .margin_end(20)
            .margin_start(20)
            .margin_top(20)
            .margin_bottom(20)
            .build();

        container.attach(&gtk::Label::new(Some("Left/Right Ear")), 0, 0, LEFT_RIGHT_WIDTH, LEFT_RIGHT_HEIGHT);
        container.attach(&left_right_slider, 0, 1, LEFT_RIGHT_WIDTH, LEFT_RIGHT_HEIGHT);

        left_right_slider.connect_value_changed(|slider| println!("Left/Right Ear Value: {}", slider.value()));
       
        // AMPLITUDE SLIDER
        const AMPLITUDE_WIDTH: i32 = 1;
        const AMPLITUDE_HEIGHT: i32 = 1;
        let amplitude_slider = gtk::Scale::builder()
            .orientation(Orientation::Vertical)
            .adjustment(&Adjustment::builder()
                        .lower(0.0)
                        .upper(1.0)
                        .step_increment(0.0)
                        .page_size(0.0)
                        .page_increment(0.0)
                        .value(0.5)
                        .build())
            .margin_top(20)
            .vexpand(true)
            .build();
        container.attach(&gtk::Label::new(Some("Amplitude")), 0, 2, AMPLITUDE_WIDTH, AMPLITUDE_HEIGHT);
        container.attach(&amplitude_slider, 0, 3, AMPLITUDE_WIDTH, AMPLITUDE_HEIGHT);

        amplitude_slider.connect_value_changed({
            let sink = sink.clone();
            move |slider| {
                sink.try_borrow().unwrap().set_volume(slider.value() as f32);
            }
        });

        // FREQUENCY SLIDER
        const FREQUENCY_WIDTH: i32 = 1;
        const FREQUENCY_HEIGHT: i32 = 1;
        let frequency_slider = gtk::Scale::builder()
            .orientation(Orientation::Vertical)
            .adjustment(
                &Adjustment::builder()
                    .lower(0.0)
                    .upper(1000.0)
                    .step_increment(0.0)
                    .page_size(0.0)
                    .page_increment(0.0)
                    .value(DEFAULT_FREQUENCY as f64)
                    .build())
            .margin_top(20)
            .vexpand(true)
            .build();
        container.attach(&gtk::Label::new(Some("Frequency")), 1, 2, FREQUENCY_WIDTH, FREQUENCY_HEIGHT);
        container.attach(&frequency_slider, 1, 3, FREQUENCY_WIDTH, FREQUENCY_HEIGHT);

        frequency_slider.connect_value_changed({
            let sink = sink.clone();
            move |slider| {
                sink.try_borrow().unwrap().clear();
                sink.try_borrow().unwrap().append(SineWave::new(slider.value() as f32));
                sink.try_borrow().unwrap().play();
            }
        });

        // finish up
        window.set_child(Some(&container));
        window.present();
    });

    application.run();
}
