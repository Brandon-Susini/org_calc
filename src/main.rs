//FUNCTIONS:
//convert a arbitrary length binary string into a decimal value
//convert a given string into a string with whitespace completely removed
//***************************Done************************************************* */
//Convert a given floating point decimal number into binary
//  ex: 13.375 = 1101 .011 **For this use floating_point_to_binary.pptx in F:\School\CompOrg

//Some of this may be better suited for JS because of quick and easy interactive elements.


//hello.rs
// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This is a very small example of how to setup a druid application.
//! It does the almost bare minimum while still being useful.

// On Windows platform, don't show a console when opening the app.
//#![windows_subsystem = "windows"]
//extern crate clipboard_win;
//use clipboard_win::Clipboard;
extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use druid::widget::prelude::*;
use druid::widget::{Flex, FlexParams,Label, TextBox, Button, Painter};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc,
            Env, PaintCtx,Rect,RenderContext,Color};


const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
struct BinaryData {
    binary_string: String,
}
impl BinaryData{
    fn new(input:String)->BinaryData{
        let mut b = BinaryData{binary_string:input};
        b
    }
    fn get_stripped_string(&self) -> String{
        let temp = self.binary_string.clone();
        temp.split_whitespace().fold("".to_string(),|mut acc,x| {acc.push_str(x); acc})
    }
    fn convert_to_decimal(&self) -> Option<i32>{
        const BASE:u32 = 10;
        let mut total:i32 = 0;
        let reversed = self.get_stripped_string().clone();
        let rev_iter = reversed.chars().rev();
        for (i,x) in rev_iter.enumerate(){
            //If x.to_digit is_none() quit and return None
            if x.to_digit(BASE).is_none() || (x.to_digit(BASE).unwrap_or(1)!=1 && x.to_digit(BASE).unwrap_or(1)!=0){return None}
            let x = x.to_digit(BASE).unwrap();
            if x == 1{
                total += 2i32.checked_pow(i.try_into().unwrap()).unwrap();
            }
        }
        return Some(total);
    }
    fn show_as_hex(&self) -> String{
        if self.convert_to_decimal().is_some(){
            let value = self.convert_to_decimal().unwrap();
            format!("{value:#x}")
        }else{
            format!("Input Invalid")
        }
    }
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Interactive Binary Calculator")
        .window_size((600.0, 600.0));

    // create the initial app state
    /*
    let initial_state: BinaryData = BinaryData {
        binary_string: "0001".into(),
    };
    */
    let initial_state: BinaryData = BinaryData::new("1".to_string());

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

//Build our root widget that will house all other widgets
fn build_root_widget() -> impl Widget<BinaryData> {
    //Hex label
    let hex_label = Label::new(|data: &BinaryData, _env: &Env|{
        format!{"Base 16: "}
    })
    .with_text_size(32.0)
    .padding((10.0,0.0))
    .align_horizontal(UnitPoint::LEFT);;
    //Hex value
    let hex_label_value = Label::new(|data: &BinaryData, _env: &Env|{
        if data.binary_string.is_empty(){
            "----".to_string()
        }else{
            format!("{}",data.show_as_hex())
        }
    })
    .with_text_size(32.0)
    //on_click takes a closure where we exclude all the values except
    //a reference to our BinaryData struct
    .on_click(|_,data: &mut BinaryData,_|{
        //Copy the the given value onto the clipboard (for windows)
        //Clipboard::new().unwrap().set_string(&data.show_as_hex());
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(data.show_as_hex()).unwrap();
    })
    .padding((10.0,0.0))
    .align_horizontal(UnitPoint::RIGHT);;

    //Decimal label
    let decimal_label = Label::new(|data: &BinaryData, _env: &Env|{
        format!{"Base 10: "}
    })
    .with_text_size(32.0)
    .padding((10.0,0.0))
    .align_horizontal(UnitPoint::LEFT);
    //Decimal value
    // a label that will determine its text based on the current app data.
    //Label new takes a reference to our data struct, and a reference to an Environment struct
    let decimal_label_value = Label::new(|data: &BinaryData, _env: &Env| {
        //Use the closure data to set the String value of our label
        if data.binary_string.is_empty() {
            "----".to_string()
        } else {
            //Get the option returned by BinaryData::convert_to_decimal
            match data.convert_to_decimal(){
                //Update label if value is good
                Some(x) => format!("{}", data.convert_to_decimal().unwrap()),
                //Set label to error message if value is None
                _ => format!("Invalid Input")
            } 
        }
    })
    .with_text_size(32.0)
    .on_click(|_,data: &mut BinaryData,_|{
        //Copy the the given value onto the clipboard (for windows)
        //Clipboard::new().unwrap().set_string(&data.convert_to_decimal().unwrap().to_string());
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(data.convert_to_decimal().unwrap().to_string()).unwrap();
    })
    .padding((10.0,0.0))
    .align_horizontal(UnitPoint::RIGHT);
    //end decimal_label_value

    let saved_label = Label::new(|data: &BinaryData,_env: &Env|{
        ""
    }).with_text_size(18.0);
    //.update(&mut self,_ctx:&UpdateCtx,_old_data:Option<&BinaryData>,data:&BinaryData,env:&Env);

    
    

    

    // a textbox that modifies `binary_string`.
    let textbox = TextBox::new()
        .with_placeholder("Binary number here!")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(BinaryData::binary_string);

    
    //Create button using its from_label method. We can do this because from_label
    //  does not reference self. Effectively meaning its like static in java.
    //from_label takes a label object for its new() constructor. 
    //Before passing it back we call with_text_size on IT because button doesn't
    //have its own with_text_size and I also couldn't figure out how to get the label
    //that exists in button.
    let save_stripped = Button::from_label(Label::new("Stripped: XXXX").with_text_size(18.0))
        .fix_width(TEXT_BOX_WIDTH)
        .on_click(|_,data: &mut BinaryData,_|{
            //Clipboard::new().unwrap().set_string(&data.get_stripped_string());
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(data.get_stripped_string()).unwrap();
            string_copied();
        })
       .fix_height(40.0);
    let save_spaced = Button::from_label(Label::new("Spaced: X-X-X-X").with_text_size(18.0))
       .fix_width(TEXT_BOX_WIDTH)
       .on_click(|_,data: &mut BinaryData,_|{
           //Clipboard::new().unwrap().set_string(&data.binary_string);
           let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
           ctx.set_contents(data.binary_string.clone()).unwrap();
           string_copied();
       })
      .fix_height(40.0);

    let info_label:Label<BinaryData> = Label::new("Enter any binary string (spaces allowed).
        \nAnything other than 0s and 1s are invalid.".to_string())
        .with_text_size(18.0);
    
    
    // arrange the two widgets vertically, with some padding
    Flex::column()
       .with_child(
            Flex::row()
                .with_child(info_label)
       )
        .with_child(
            Flex::row()
                .with_child(saved_label)
        )
        .with_child(
            Flex::row()
                .with_flex_child(hex_label,1.0)
                .with_flex_child(hex_label_value,1.0)
                .padding(10.0)
        )
        .with_child(
            Flex::row()
            .with_flex_child(decimal_label,1.0)
            .with_flex_child(decimal_label_value,1.0)
            .padding(10.0)
        )
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(save_stripped)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(save_spaced)
        .must_fill_main_axis(true)
        .align_vertical(UnitPoint::CENTER)
        //rows a little shorter
        //Text box aligned right

        
        
}
fn string_copied(){
    println!("A string was copied!");
}
/*
    TODO: Add two labels
    *Feedback label, says something like "Value copied to clipboard" after a click
    *Find a way to map values in ranges so you can play with fontsize and stuff.
*/
/*
    let my_painter = Painter::new(|ctx, data: &HelloState,env|{
        let bounds = Rect::new(0.0,0.0,100.0,100.0);
        println!("{:?}",ctx.size());
        ctx.fill(bounds,&env.get(druid::theme::PRIMARY_DARK));
    });
    */