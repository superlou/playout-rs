use gtk;
use relm::{Widget};
use gtk::{WidgetExt, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, IsA};

pub trait Style
    where Self: Widget,
          Self::Root: IsA<gtk::Widget>,
          Self::Root: IsA<gtk::Object>
{
    fn add_class(&self, class: &str) {
        let style_context = self.root().get_style_context().unwrap();
        style_context.add_class(class);
    }

    fn remove_class(&self, class: &str) {
        let style_context = self.root().get_style_context().unwrap();
        style_context.remove_class(class);
    }

    fn add_stylesheet(&self, style: &str) {
        let style_context = self.root().get_style_context().unwrap();
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}
