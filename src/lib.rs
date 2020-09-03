use bevy::prelude::*;
use bevy_utils::HashMap;
use bevy_window::{WindowPlugin, WindowCreated};
use bevy_glutin::{GlutinPlugin, GlutinWindows};
use bevy_input::InputPlugin;
use bevy_core::CorePlugin;
use bevy_type_registry::TypeRegistryPlugin;
use glium::backend::glutin::raw::Raw;
use std::sync::{Arc, Mutex};

pub use bevy_glutin::GlutinContexts;
pub use bevy_window::WindowId;

pub use glium;

pub struct GliumPlugin;

impl Plugin for GliumPlugin{
    fn build (&self, app: &mut AppBuilder){
        app
        .add_plugin(TypeRegistryPlugin::default())
        .add_plugin(CorePlugin::default())
        .add_plugin(GlutinPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_plugin(InputPlugin::default())
        .init_resource::<GliumDisplays>()
        .add_stage("window_create")
        .add_system_to_stage("window_create", handle_window_created.thread_local_system());
    }
}

fn handle_window_created(world: &mut World, resources: &mut Resources){

    let mut contexts = resources.get_mut::<GlutinContexts>().unwrap();
    let mut displays = resources.get_mut::<GliumDisplays>().unwrap();
    let create_window_events = resources.get_mut::<Events<WindowCreated>>().unwrap();

    for window_created in create_window_events.get_reader().iter(&create_window_events){
        displays.create_raw(&mut contexts, window_created.id);
    }
}

unsafe impl Send for GliumDisplays{}
unsafe impl Sync for GliumDisplays{}
#[derive(Default)]
pub struct GliumDisplays{
    displays: HashMap<WindowId, Raw>,
}

impl GliumDisplays{
    /// Create a Raw glium rendering handle.  ONLY USE ON LOCAL THREAD
    fn create_raw(&mut self, contexts: &mut RefMut<GlutinContexts>, id: WindowId) -> Result<(), ()>{

        let context = match contexts.take_context(id){
            Some(context) => unsafe{context.make_current().unwrap()},
            None => Err(())?,
        };
    
        let display = 
            Raw::from_gl_window(context).unwrap();
        
        self.displays.insert(id, display);
        Ok(())
    }
    
    /// Create a Raw glium rendering handle.  ONLY USE ON LOCAL THREAD
    pub fn get(&self, id: WindowId) -> Option<&Raw>{
        self.displays.get(&id)
    }
}
