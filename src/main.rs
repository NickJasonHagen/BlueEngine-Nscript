#![allow(unused)]

use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::default_resources::DEFAULT_MATRIX_4,
    Engine, Instance, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};

use blue_engine_utilities::FlyCamera;
//use nscript_v2::*;
//extern crate nscript_v2;
use std::collections::{HashMap};
//use std::{env, array, string};
extern crate nscript;
use nscript::*;
fn main() {

    let mut vmap = Varmap::new(); // global
    let mut engine = Engine::new().expect("win");
    //let mut engine = ENGINE.lock().unwrap();
    //let test_instance = Instance::default();
    //println!("{:?}", test_instance.to_raw());
let mut texture_map: HashMap<String, blue_engine::wgpu::BindGroup> = HashMap::new();


    let texturenotfound = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/unknowntexture.png".to_owned()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();
texture_map.insert("unknowntexture.png".to_owned(), texturenotfound);


    let texture = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/BlueLogoDiscord.png".to_owned()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();

        //cwrite(&texture,"");
    let texture2 = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/player.png".to_owned()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();

    let texture3 = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/image.png".to_owned()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();

    square(
        "main",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    );

    engine.objects.get_mut("main").unwrap().set_texture(texture);
    engine
        .objects
        .get_mut("main")
        .unwrap()
        .set_position(-1f32, 1f32, 0f32);

    square(
        "alt",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    );
    engine.objects.get_mut("alt").unwrap().set_texture(texture2);
    engine
        .objects
        .get_mut("alt")
        .unwrap()
        .set_position(0.2f32, 0.1f32, 0.001f32);

    square(
        "alt2",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    );
   engine
        .objects
        .get_mut("alt2")
        .unwrap()
        .set_texture(texture3);
    engine
        .objects
        .get_mut("alt2")
        .unwrap()
        .set_position(-0.2f32, -0.2f32, 0.001f32);

    let speed = -0.05;


    let serverscriptfilename = SCRIPT_DIR.to_owned() +"system/BENC.nc";
    nscript_execute_script(&serverscriptfilename,"","","","","","","","","",&mut vmap);
    // spanwning quee
    for i in split(&nscript_checkvar("blueengine.square_quee", &mut vmap),NC_ARRAY_DELIM){
        cwrite(&i,"green");
        if i != "" {
        square(
            i,
            ObjectSettings::default(),
            &mut engine.renderer,
            &mut engine.objects,
        );
            cwrite(&i,"yellow")
        }
    }
    // set positions quee
    for i in split(&nscript_checkvar("blueengine.position_quee", &mut vmap),NC_ARRAY_DELIM){
        //cwrite(&i,"green");
        if i != "" {
            let data = split(&i,",");
            if data.len() > 2 {
cwrite(&i,"yellow");
                engine
                    .objects
                    .get_mut(data[0])
                    .unwrap()
                    .set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0));

            }
            else{
                cwrite("A split error on the position quee accuired","red")

            }
        }
    }


    // quee system to load textures , ( first nodes can be used to copy to others)
    for i in split(&nscript_checkvar("blueengine.texture_quee", &mut vmap),NC_ARRAY_DELIM){
        let ckey = "blueengine_textures.".to_owned() + i;
        if nscript_checkvar(&ckey, &mut vmap) == "" && i != ""{
            cwrite(&ckey,"green");
            vmap.setvar("tmp".to_owned(),i);
            if i != "" {
                //perform blue engine texture loading
                let t = engine
                    .renderer
                    .build_texture(
                        "background",
                        TextureData::Path(i.to_owned()),
                        blue_engine::TextureMode::Clamp,
                    )
                    .unwrap();
                // create a object with the texture to be cloned
                square(
                    ckey.clone()
                    ,
                    ObjectSettings::default(),
                    &mut engine.renderer,
                    &mut engine.objects,
                );
                // set the main texture !
                engine.objects.get_mut(&ckey).unwrap().set_texture(t);

                // move the object out of sight ( yeah i know blue it will be invisible :P )
                engine
                    .objects
                    .get_mut(&ckey)
                    .unwrap()
                    .set_position(-0.2f32, 0.1f32, 0.001f32);
                // let system know its used!
                vmap.setvar(ckey.clone(), &ckey);
                let m = "texture added:".to_owned() + &ckey;
                cwrite(&m,"g")
            }


        }
        else{
            let m = "texture already exists:".to_owned() + &ckey;
            cwrite(&m,"r")
        }
    }



    cwrite(&nscript_checkvar("blueengine.squarequee", &mut vmap),"red");

// key mapping, used inside game_loop for bridging to nscript
   let keyvec = [
    blue_engine::VirtualKeyCode::Up,
    blue_engine::VirtualKeyCode::Down,
    blue_engine::VirtualKeyCode::Left,
    blue_engine::VirtualKeyCode::Right,
    blue_engine::VirtualKeyCode::A,
    blue_engine::VirtualKeyCode::B,
    blue_engine::VirtualKeyCode::C,
    blue_engine::VirtualKeyCode::D,
    blue_engine::VirtualKeyCode::E,
    blue_engine::VirtualKeyCode::F,
    blue_engine::VirtualKeyCode::G,
    blue_engine::VirtualKeyCode::H,
    blue_engine::VirtualKeyCode::I,
    blue_engine::VirtualKeyCode::J,
    blue_engine::VirtualKeyCode::K,
    blue_engine::VirtualKeyCode::L,
    blue_engine::VirtualKeyCode::M,
    blue_engine::VirtualKeyCode::N,
    blue_engine::VirtualKeyCode::O,
    blue_engine::VirtualKeyCode::P,
    blue_engine::VirtualKeyCode::Q,
    blue_engine::VirtualKeyCode::R,
    blue_engine::VirtualKeyCode::S,
    blue_engine::VirtualKeyCode::T,
    blue_engine::VirtualKeyCode::U,
    blue_engine::VirtualKeyCode::V,
    blue_engine::VirtualKeyCode::W,
    blue_engine::VirtualKeyCode::X,
    blue_engine::VirtualKeyCode::Y,
    blue_engine::VirtualKeyCode::Z,];
    let keyname = [ // keymapping naming ( must contain the same size and order as the keymapping!!)
    "key.up",
    "key.down",
    "key.left",
    "key.right",
    "key.a",
    "key.b",
    "key.c",
    "key.d",
    "key.e",
    "key.f",
    "key.g",
    "key.h",
    "key.i",
    "key.j",
    "key.k",
    "key.l",
    "key.m",
    "key.n",
    "key.o",
    "key.p",
    "key.q",
    "key.r",
    "key.s",
    "key.t",
    "key.u",
    "key.v",
    "key.w",
    "key.x",
    "key.y",
    "key.z",
];




    engine
        .update_loop(move |renderer, window, objects, input, camera, plugins| {

            // run all nscript loops , (quees be filled here and bridged to the render engine)
            nscript_loops(&mut vmap);
            vmap.setvar("key.event".to_owned(),"false");


            // Bridge: Nscript Scale Quee handler.
            // in nscript class blueengine.roation_q is used by func blueengine.setscale()
            let positionquee = nscript_checkvar("blueengine.scale_q", &mut vmap);
            for i in split(&positionquee,NC_ARRAY_DELIM){
                if i != ""{ // if queed items in pool

                    let data = split(&i,",");
                    if data.len() > 2 && data[0] != ""{// check obj,x,y,z
                        objects
                            .get_mut(data[0])
                            .unwrap()
                            .resize(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0),window.inner_size());
                    }
                    else{
                        cwrite("Split error on the position q","red")
                    }
                    // remove the entree from the pool ( in nscript blueengine.position_quee )
                    vmap.setvar("blueengine.scale_q".to_owned(),&poolremove(&positionquee,&i) );

                }
            }

            // Bridge: Nscript scale Quee handler.
            // in nscript class blueengine.position_q is used by func blueengine.setposition()

            let positionquee = nscript_checkvar("blueengine.position_q", &mut vmap);
            for i in split(&positionquee,NC_ARRAY_DELIM){
                if i != ""{ // if queed items in pool

                    let data = split(&i,",");
                    if data.len() > 2 && data[0] != ""{// check obj,x,y,z
                        objects
                            .get_mut(data[0])
                            .unwrap()
                            .set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0));
                    }
                    else{
                        cwrite("Split error on the position quee","red")
                    }
                    // remove the entree from the pool ( in nscript blueengine.position_quee )
                    vmap.setvar("blueengine.position_q".to_owned(),&poolremove(&positionquee,&i) );

                }
            }

            // Bridge: Nscript rotation Quee handler.
            // in nscript class blueengine.roation_q is used by func blueengine.setrotatation()
            let positionquee = nscript_checkvar("blueengine.rotation_q", &mut vmap);
            for i in split(&positionquee,NC_ARRAY_DELIM){
                if i != ""{ // if queed items in pool
                    let data = split(&i,",");
                    if data.len() > 2 {// check obj,x,y,z
                        let axis = match data[2]{
                            "x" => {
                                RotateAxis::X
                            }
                            "y" => {
                                RotateAxis::Y
                            }
                            "z" => {
                                RotateAxis::Z
                            }
                            _ => RotateAxis::X
                        };
                        objects
                            .get_mut(data[0])
                            .unwrap()
                            .set_rotatation(data[1].parse().unwrap_or(0.0), axis);
                    }
                    else{
                        cwrite("Split error on the rotation quee ","red")
                    }
                    // remove the entree from the pool ( in nscript blueengine.position_quee )
                    vmap.setvar("blueengine.rotation_q".to_owned(),&poolremove(&positionquee,&i) );

                }
            }

            // Keyboard-input bridge , registers key.* classproperties to down or up
            let mut idx = 0;
            for xkey in keyname{
                if input.key_held(keyvec[idx]) {
                    vmap.setvar(keyname[idx].to_owned(),"down");
                    vmap.setvar("key.event".to_owned(),"true");
                    //cwrite(&keyname[idx],"y")
                }
                else{
                    vmap.setvar(keyname[idx].to_owned(),"up");
                }
                idx +=1;
            }
            //reset nscript property
        })
        .expect("Error during update loop");
}






