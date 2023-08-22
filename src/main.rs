use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;
use cacao::button::Button;

use cacao::layout::LayoutConstraint;
use cacao::text::Label;
use cacao::layout::Layout;
use cacao::view::View;


struct AppDelegateImpl {
    window: Window,
    label: Label,
    view: View,
    button: Button,
}

impl Default for AppDelegateImpl {
    fn default() -> Self {
        Self {
            window: Window::default(),
            label: Label::default(),
            view: View::default(),
            button: Button::new("New Button!"),
        }
    }
}

impl AppDelegate for AppDelegateImpl {

    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.view.add_subview(&self.label);
        self.view.add_subview(&self.button);
        self.window.set_content_view(&self.view);
        let vertical_padding :f32 = 10.0;
        LayoutConstraint::activate(&[
            self.label.center_x.constraint_equal_to(&self.view.center_x),
            self.label.center_y.constraint_equal_to(&self.view.center_y),
            self.button.center_x.constraint_equal_to(&self.view.center_x),
            self.button.top.constraint_greater_than_or_equal_to(&self.label.bottom),

        ]);

        self.label.set_text("Label!");
        self.window.set_title("Куку!");
        self.window.show();
    }
}

fn main() {
    App::new("com.hello.world", AppDelegateImpl::default()).run();
}