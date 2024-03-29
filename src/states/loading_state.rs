// common modules
use std::fs::read_dir;

// amethyst modules
use amethyst::{
    prelude::*,
    ecs::Entity,
    assets::{
        Completion, 
        ProgressCounter,
        Handle,
        // PrefabLoader,
        // RonFormat,
    },
    ui::{
        UiLoader,
        UiPrefab,
        UiFinder,
    },
    utils::application_root_dir,
};

// local modules
use crate::components::ui_glowing_comp::UiGlowingStyle;
use crate::states::disclaimer_state::DisclaimerState;
use crate::resources::ui_prefab_registry::UiPrefabRegistry;
use crate::resources::ui_helper::impl_glowing_comp;
use crate::resources::audio::initialize_audio;

//===========
// Constants
//===========
const LOADING_SCREEN_ID: &str = "loading_screen";
const LOADING_TEXT_ID:   &str = "loading_label";

//=======================
// Declare loading state
//=======================
//
// Note that if it is not a unit struct (with no fields)
// you cannot directly use it as the parameter of the Application::new() function
// a seperate method (here we use default())to return an instance (Self) need to be used
#[derive(Default)]
pub struct LoadingState {
    // Tracks loaded assets.
    loading_screen_progress:    Option<ProgressCounter>,
    loading_prefabs_progress:   Option<ProgressCounter>,
    loading_screen:             Option<Entity>,
    loading_screen_text:        Option<Entity>,
    loading_screen_is_ready:    bool,
}

//=======================
// Implement State trait
//=======================
impl SimpleState for LoadingState {

    //----------------
    // Start up tasks
    //----------------
    fn on_start(&mut self, mut data: StateData<GameData>) {
        initialize_audio(data.world);
        let mut ui_prefab_registry = UiPrefabRegistry::default();
        self.loading_screen_progress    = Some(load_loading_screen(&mut data.world, &mut ui_prefab_registry));
        self.loading_prefabs_progress   = Some(load_prefabs(&mut data.world, &mut ui_prefab_registry));
        data.world.insert(ui_prefab_registry);
    }

    //----------------
    // Stopping tasks 
    //----------------
    fn on_stop(&mut self, data: StateData<GameData>) {
        // clean up
        self.loading_screen_is_ready    = false;
        self.loading_screen_progress    = None;
        self.loading_prefabs_progress   = None;
        // remove loading screen
        if let Some(loading_screen) = self.loading_screen {
            if data.world.delete_entity(loading_screen).is_ok() {
                self.loading_screen_text = None;
                self.loading_screen      = None;
            }
        }

    }

    //--------------
    // Update tasks 
    //--------------
    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {

        // update game data
        data.data.update(&data.world);

        // show loading screen when completed
        if self.loading_screen.is_none() {
            if let Some(ref load_screen_prog) = self.loading_screen_progress.as_ref() {
                match load_screen_prog.complete() {
                    Completion::Loading  => {
                        return Trans::None;
                    }
                    Completion::Failed   => {
                        error!("Loading Screen Failed to Load!");
                        return Trans::Quit;
                    }
                    Completion::Complete => {
                        let loading_scrn = data
                            .world
                            .read_resource::<UiPrefabRegistry>()
                            .find(data.world, LOADING_SCREEN_ID);
                        if let Some(loading_scrn) = loading_scrn {
                            self.loading_screen = Some(
                                data.world.create_entity()
                                    .with(loading_scrn)
                                    .build()
                            );
                        }
                    }
                }           
            }
        } else if !self.loading_screen_is_ready {
            self.loading_screen_text = data.world.exec(|ui_finder: UiFinder<'_>| {
                ui_finder.find(LOADING_TEXT_ID) 
            });
            if let Some(loading_text) = self.loading_screen_text {   
                impl_glowing_comp(
                    &loading_text, 
                    data, 
                    true, 
                    1., 
                    0.8, 
                    UiGlowingStyle::Darkening, 
                    [1., 1., 0., 0.]
                );
            }
            self.loading_screen_is_ready = true;
        } 

        if let Some(ref load_prefabs_prog) = self.loading_prefabs_progress.as_ref() {
            match load_prefabs_prog.complete() {
                Completion::Loading  => {
                    return Trans::None;
                }
                Completion::Failed   => {
                    error!("Prefabs Failed to Load!");
                    return Trans::Quit;
                }
                Completion::Complete => {
                    return Trans::Switch(Box::new(DisclaimerState::default()));
                }
            }
        } 
        
        Trans::None
    }
}

// Load Loading screen in another thread, and register it
fn load_loading_screen(world: &mut World, registry:&mut UiPrefabRegistry) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();
    let laoding_screen_path = application_root_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/resources/prefabs/ui_loading/loading_screen.ron";

    registry.prefabs.push(world.exec(|loader: UiLoader<'_>| {
        loader.load(
            laoding_screen_path,
            &mut progress_counter,
        )
    }));
    progress_counter  
}

// Load Prefabs in another thread, and register it
fn load_prefabs(world: &mut World, registry:&mut UiPrefabRegistry) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();

    // UI Prefabs
    let ui_prefab_dir_path = application_root_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/resources/prefabs/ui";
    let ui_prefab_iter = read_dir(ui_prefab_dir_path).unwrap();
    registry.prefabs.extend(ui_prefab_iter
        .map(|prefab_dir_entry| {
            world.exec(|loader: UiLoader<'_>| {
                loader.load(
                    make_name("prefabs/ui/", &prefab_dir_entry.unwrap()),
                    &mut progress_counter,
                )
            })
        })
        .collect::<Vec<Handle<UiPrefab>>>());
    
    // Paddle Prefabs
    // let game_prefab_iter = {
    //     let game_prefab_dir_path = application_root_dir()
    //         .unwrap()
    //         .into_os_string()
    //         .into_string()
    //         .unwrap()
    //         + "/resources/prefabs/paddles";
    //     let game_prefab_iter_temp = read_dir(game_prefab_dir_path).unwrap();
    //     game_prefab_iter_temp.map(|game_prefab_dir_entry|{
    //         world.exec(|loader: PrefabLoader<'_, PaddlePrefabData>| {
    //             loader.load(
    //                 make_name("prefabs/paddles/", &game_prefab_dir_entry.unwrap()),
    //                 RonFormat,
    //                 &mut progress_counter,
    //             )
    //         })
    //     })
    // };

    progress_counter
}

fn make_name(subdirectory: &str, entry: &std::fs::DirEntry) -> String {
    let path_buffer = entry.path();
    let filename = path_buffer.file_name().unwrap();
    format!("{}{}", subdirectory, filename.to_str().unwrap())
}