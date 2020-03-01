//! This program was automatically generated by Visual Embedded Rust. Don't edit here!
extern crate macros as mynewt_macros;   //  Declare the Mynewt Procedural Macros library
use mynewt_macros::infer_type;          //  Import Mynewt procedural macros
use mynewt::{
  result::*,              // Import Mynewt API Result and Error types
  sys::console,           // Import Mynewt Console API
};
use druid::{
  AppLauncher, Data, EventCtx, LocalizedString, Widget, WindowDesc,
  widget::{
      Align, Button, Column, Label, Padding,
  },
  argvalue::ArgValue,
  env::Env,
};

/// Application State
#[infer_type]  //  Infer the missing types
#[derive(Clone, Data, Default)]
struct State {
    count: _,
}

/// Will be run upon startup to launch the app
#[infer_type]  //  Infer the missing types
pub fn on_start() -> MynewtResult<()> {
    console::print("on_start\n");
    //  Build a new window
    let main_window = WindowDesc::new(ui_builder);
    //  Create application state
    let mut state = State::default();
    state.count = 0;

    //  Launch the window with the application state
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)
        .expect("launch failed");
    //  Return success to `main()` function
    Ok(())
}

/// Build the UI for the window
#[infer_type]  //  Infer the missing types
fn ui_builder() -> impl Widget<State> {  //  `State` is the Application State
    console::print("Rust UI builder\n"); console::flush();
    //  Create a line of text
    let my_label_text = LocalizedString::new("hello-counter")
        .with_arg("count", on_my_label_show);  //  Call `on_my_label_show` to get label text
    //  Create a label widget `my_label`
    let my_label = Label::new(my_label_text);
    //  Create a button widget `my_button`
    let my_button = Button::new("Visual Rust", on_my_button_press);  //  Call `on_my_button_press` when pressed

    //  Create a column
    let mut col = Column::new();
    //  Add the label widget to the column, centered with padding
    col.add_child(
        Align::centered(
            Padding::new(5.0,
                my_label
            )
        ),
        1.0
    );
    //  Add the button widget to the column, with padding
    col.add_child(
        Padding::new(5.0,
            my_button
        ),
        1.0
    );
    //  Return the column containing the widgets
    col
}  //  ;

/// Callback function that will be called to create the formatted text for the label `my_label`
#[infer_type]  //  Infer the missing types
fn on_my_label_show(state: _, env: _) -> ArgValue {
    console::print("on_my_label_show\n");
    state.count.into()
}

/// Callback function that will be called when the button `my_button` is pressed
#[infer_type]  //  Infer the missing types
fn on_my_button_press(ctx: _, state: _, env: _) {
    console::print("on_my_button_press\n");
    state.count = state.count + 1;
}

//  -- BEGIN BLOCKS --<xml xmlns="http://www.w3.org/1999/xhtml"><variables><variable type="" id="7gmf.o0opM2*Y0$95Xv*">count</variable></variables><block type="on_start" id="3zi/F838J{]z`u2sOwAy" x="38" y="13"><statement name="STMTS"><block type="variables_set" id="u;:BgkEw07cVd%QWWy|_"><field name="VAR" id="7gmf.o0opM2*Y0$95Xv*" variabletype="">count</field><value name="VALUE"><block type="math_number" id=",@3g:]LmR.jMQ?dkOVeo"><field name="NUM">0</field></block></value></block></statement></block><block type="app" id="a+fa^[wDegFmxc]0oS)*" x="38" y="88"><mutation items="3"></mutation><value name="ADD0"><block type="label" id="iP(ppbW~bgVwE{uY3ZCF"><field name="NAME">my_label</field><field name="PADDING">5</field></block></value><value name="ADD1"><block type="button" id="C9k`r{IwfwWbtEqn9tw!"><field name="NAME">my_button</field><field name="TITLE">Visual Rust</field><field name="PADDING">5</field></block></value></block><block type="widgets_defreturn" id="}ggypf|bz!fTICk_[bN]" x="38" y="187"><field name="NAME">my_label</field><value name="RETURN"><block type="variables_get" id="5Qfvt?a#qJ]ow$7{/F.|"><field name="VAR" id="7gmf.o0opM2*Y0$95Xv*" variabletype="">count</field></block></value></block><block type="widgets_defnoreturn" id="on[)x(-*wwqqpST6`+$}" x="37" y="313"><field name="NAME">my_button</field><statement name="STACK"><block type="variables_set" id="+`t4VSP2GAe%4nSdW40I"><field name="VAR" id="7gmf.o0opM2*Y0$95Xv*" variabletype="">count</field><value name="VALUE"><block type="math_arithmetic" id="wF(J#r9b-l5W5k$*-sg,"><field name="OP">ADD</field><value name="A"><shadow type="math_number" id="X/gy5eKt6K.Z.[TfIdI`"><field name="NUM">1</field></shadow><block type="variables_get" id="b[t|v|7RR73g)rSpp`89"><field name="VAR" id="7gmf.o0opM2*Y0$95Xv*" variabletype="">count</field></block></value><value name="B"><shadow type="math_number" id=",)krx?Q@g{CengYXN}n#"><field name="NUM">1</field></shadow></value></block></value></block></statement></block></xml>-- END BLOCKS --