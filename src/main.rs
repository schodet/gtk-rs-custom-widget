//! # Proof of concept for Scrollable implementation.
//!
//! The aim of this POC is to derive a widget from DrawingArea, implementing the Scrollable
//! interface so that it can handle the scrolling itself while contained in a ScrolledWindow.
//!
//! This could be used for example for an oscilloscope like widget where scrolling is used to
//! navigate inside a recorded signal.
//!
//! There is another option to implement this: embed a giant DrawingArea inside a ViewPort. There
//! is however a limit on the size of the DrawingArea, and therefore on the zooming level of the
//! scope.  The advantage of this solution is that GTK is doing a lot of work for us, like the
//! pixelcache which allows better performances.
//!
//! A third solution would be to not use the ScrolledWindow and use a separated ScrollBar. In this
//! case, our widget can connect to the changed signal and scroll.  However, this is not the way it
//! is supposed to be done and scrolling with a touchscreen would not work for example.
//!
//! # Does it work yet?
//!
//! No.
//!
//! I have got the following errors at runtime:
//!
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.074: validate_and_install_class_property: assertion 'class->set_property != NULL' failed
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.074: validate_and_install_class_property: assertion 'class->set_property != NULL' failed
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.074: validate_and_install_class_property: assertion 'class->set_property != NULL' failed
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.074: validate_and_install_class_property: assertion 'class->set_property != NULL' failed
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.075: Object class MyWidget doesn't implement property 'vscroll-policy' from interface 'GtkScrollable'
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.075: Object class MyWidget doesn't implement property 'vadjustment' from interface 'GtkScrollable'
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.075: Object class MyWidget doesn't implement property 'hscroll-policy' from interface 'GtkScrollable'
//!     (gtk-rs-custom-widget:4447): GLib-GObject-CRITICAL **: 23:50:33.075: Object class MyWidget doesn't implement property 'hadjustment' from interface 'GtkScrollable'
//!
//! It looks like the set_property method is not set yet when the Scrollable interface is added.

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Unable to start GTK3. Error");
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let widget = custom::MyWidget::new();
    widget.connect_draw(|_, cr| {
        cr.set_line_width(1.0);
        cr.rectangle(10.5, 10.5, 500.0, 500.0);
        cr.stroke();
        Inhibit(false)
    });
    window.add(&widget);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_all();
    gtk::main();
}

mod custom {

    use glib::glib_object_impl;
    use glib::glib_object_subclass;
    use glib::glib_object_wrapper;
    use glib::glib_wrapper;
    use glib::object::Cast;
    use glib::subclass;
    use glib::translate::*;
    use glib::types::StaticType;
    use glib::value::ToValue;
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;

    mod imp {

        use super::*;

        /* As properties could (must?) be overridden, this is not needed.
        static PROPERTIES: [subclass::Property; 4] = [
            subclass::Property("hadjustment", |name| {
                glib::ParamSpec::object(
                name,
                "Horizontal adjustment",
                "Horizontal adjustment that is shared between the scrollable widget and its controller",
                gtk::Adjustment::static_type(),
                glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT,
                )
            }),
            subclass::Property("vadjustment", |name| {
                glib::ParamSpec::object(
                name,
                "Vertical adjustment",
                "Vertical adjustment that is shared between the scrollable widget and its controller",
                gtk::Adjustment::static_type(),
                glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT,
                )
            }),
            subclass::Property("hscroll-policy", |name| {
                glib::ParamSpec::enum_(
                name,
                "Horizontal Scrollable Policy",
                "How the size of the content should be determined",
                gtk::ScrollablePolicy::static_type(),
                gtk::ScrollablePolicy::Minimum.to_glib(),
                glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY,
                )
            }),
            subclass::Property("vscroll-policy", |name| {
                glib::ParamSpec::enum_(
                name,
                "Vertical Scrollable Policy",
                "How the size of the content should be determined",
                gtk::ScrollablePolicy::static_type(),
                gtk::ScrollablePolicy::Minimum.to_glib(),
                glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY,
                )
            }),
        ];
        */

        pub struct MyWidget {
            hadjustment: RefCell<Option<gtk::Adjustment>>,
            vadjustment: RefCell<Option<gtk::Adjustment>>,
            hscroll_policy: RefCell<gtk::ScrollablePolicy>,
            vscroll_policy: RefCell<gtk::ScrollablePolicy>,
        }

        impl ObjectSubclass for MyWidget {
            const NAME: &'static str = "MyWidget";

            type ParentType = gtk::DrawingArea;

            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            fn type_init(type_: &mut subclass::InitializingType<Self>) {
                type_.add_interface::<gtk::Scrollable>();
            }

            fn class_init(klass: &mut Self::Class) {
                //klass.install_properties(&PROPERTIES);
                // Use integer as a quick and dirty solution for now.
                klass.override_property("hadjustment", 1);
                klass.override_property("vadjustment", 2);
                klass.override_property("hscroll-policy", 3);
                klass.override_property("vscroll-policy", 4);
            }

            fn new() -> Self {
                Self {
                    hadjustment: RefCell::new(None),
                    vadjustment: RefCell::new(None),
                    hscroll_policy: RefCell::new(gtk::ScrollablePolicy::Minimum),
                    vscroll_policy: RefCell::new(gtk::ScrollablePolicy::Minimum),
                }
            }
        }

        impl ObjectImpl for MyWidget {
            glib_object_impl!();
            fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
                //let prop = &PROPERTIES[id];
                //match *prop {
                match id {
                    //subclass::Property("hadjustment", ..) => {
                    1 => {
                        let hadjustment = value.get().unwrap();
                        self.hadjustment.replace(hadjustment);
                    }
                    //subclass::Property("vadjustment", ..) => {
                    2 => {
                        let vadjustment = value.get().unwrap();
                        self.vadjustment.replace(vadjustment);
                    }
                    //subclass::Property("hscroll-policy", ..) => {
                    3 => {
                        let policy = value.get().unwrap().unwrap();
                        self.hscroll_policy.replace(policy);
                    }
                    //subclass::Property("vscroll-policy", ..) => {
                    4 => {
                        let policy = value.get().unwrap().unwrap();
                        self.vscroll_policy.replace(policy);
                    }
                    _ => unimplemented!(),
                }
            }
            fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
                //let prop = &PROPERTIES[id];
                //match *prop {
                match id {
                    //subclass::Property("hadjustment", ..) => {
                    1 => {
                        Ok(self.hadjustment.borrow().to_value())
                    }
                    //subclass::Property("vadjustment", ..) => {
                    2 => {
                        Ok(self.vadjustment.borrow().to_value())
                    }
                    //subclass::Property("hscroll-policy", ..) => {
                    3 => {
                        Ok(self.hscroll_policy.borrow().to_value())
                    }
                    //subclass::Property("vscroll-policy", ..) => {
                    4 => {
                        Ok(self.vscroll_policy.borrow().to_value())
                    }
                    _ => unimplemented!(),
                }
            }
        }

        impl WidgetImpl for MyWidget {}

        impl DrawingAreaImpl for MyWidget {}

        unsafe impl IsImplementable<MyWidget> for gtk::Scrollable {
            unsafe extern "C" fn interface_init(
                _iface: glib_sys::gpointer,
                _iface_data: glib_sys::gpointer,
            ) {
            }
        }
    }

    glib_wrapper! {
        pub struct MyWidget(Object<subclass::simple::InstanceStruct<imp::MyWidget>, subclass::simple::ClassStruct<imp::MyWidget>, MyWidgetClass>)
            @extends gtk::Widget, gtk::DrawingArea,
            @implements gtk::Scrollable;

        match fn {
            get_type => || imp::MyWidget::get_type().to_glib(),
        }
    }

    impl MyWidget {
        pub fn new() -> MyWidget {
            glib::Object::new(MyWidget::static_type(), &[])
                .expect("Failed to create my widget")
                .downcast()
                .expect("Created my widget is of wrong type")
        }
    }
}
