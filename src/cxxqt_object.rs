#[cxx_qt::bridge]

pub mod qobject {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.sh");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type MyObject = super::MyObjectRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        fn say_hi(self: &MyObject, string: &QString, number: i32);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct MyObjectRust {
    number: i32,
    string: QString,
}

impl qobject::MyObject {
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi form Rust! String is '{string}' and number is {number}");
    }
}
