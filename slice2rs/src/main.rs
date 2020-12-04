use ice_rs::errors::Error;
use ice_rs::slice::{
    module::Module,
    enumeration::Enum,
    struct_decl::Struct,
    types::IceType,
    interface::Interface,
    function::Function
};
use clap::Clap;
use std::path::Path;


#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    // slice_file: String
    out_dir: String
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    let mut root = Module::root();
    let rust_demo = root.add_sub_module("RustDemo")?;

    let mut rect_type = Enum::new("RectType");
    rect_type.add_variant("Rect", None);
    rect_type.add_variant("Square", None);
    rust_demo.add_enum(&rect_type);

    let mut rect = Struct::new("Rect");
    rect.add_member("left", IceType::LongType);
    rect.add_member("right", IceType::LongType);
    rect.add_member("top", IceType::LongType);
    rect.add_member("bottom", IceType::LongType);
    rust_demo.add_struct(&rect);

    let mut rect_props = Struct::new("RectProps");
    rect_props.add_member("width", IceType::LongType);
    rect_props.add_member("height", IceType::LongType);
    rect_props.add_member("rect_type", IceType::CustomType(String::from("RectType")));
    rust_demo.add_struct(&rect_props);

    let mut demo = Interface::new("Demo");

    let demo_say_hello = Function::new("sayHello", IceType::VoidType);
    demo.add_function(demo_say_hello);

    let mut demo_say_with_arg = Function::new("say", IceType::VoidType);
    demo_say_with_arg.add_argument("text", IceType::StringType);
    demo.add_function(demo_say_with_arg);

    let mut demo_calc_rect = Function::new("calcRect", IceType::CustomType(String::from("RectProp")));
    demo_calc_rect.add_argument("rc", IceType::CustomType(String::from("Rect")));
    demo.add_function(demo_calc_rect);

    rust_demo.add_interface(&demo);

    root.write(Path::new(&opts.out_dir), "demo")
}