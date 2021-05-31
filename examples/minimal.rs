use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rhai::*;
use rhai::*;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        //Adding the plugin
        .add_plugin(BevyRhaiPlugin)
        //This system request the loading of the script file and spawns an entity
        .add_startup_system(startup.system())
        //If we didn't add an engine resource yet(this is checked by the run criteria),
        //we should check if the scripts we requested to load were loaded
        .add_system(
            wait_for_load
                .system()
                .with_run_criteria(added_engine_res_run_criteria.system()),
        )
        //This system increment the component B for our entity from inside a script
        .add_system(increment_in_script.system())
        .run();
}
///This component was added just to show that you can send to the engine,
///and receive back from the engine more than one component
#[derive(Debug, Clone)]
pub struct A(pub i32);
///This component has a function attached to it, because we want to be able to call this function,
///we need to both register the type B in the engine, and its functions we want to expose to scripting
#[derive(Debug, Clone)]
pub struct B(pub f32);
impl B {
    pub fn increment(&mut self) {
        self.0 += 1.0;
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Load the script like you'd load any asset
    commands.insert_resource::<Handle<RhaiScript>>(asset_server.load("scripts/minimal.rhai"));
    //Spawning an entity
    commands.spawn_bundle((A(5), B(6.0)));
}
fn added_engine_res_run_criteria(engine_res: Option<Res<Engine>>) -> ShouldRun {
    //If we already added the engine to the resources, we shouldn't run on_load anymore
    if engine_res.is_some() {
        ShouldRun::No
    }
    //We didn't add it yet, we need to check if the script was loaded again
    else {
        ShouldRun::Yes
    }
}
///A resource that holds our engine, and the AST it compiled
struct ScriptingEngine(Engine, AST);
//This function adds the ScriptingEngine resource when the script is loaded
fn wait_for_load(
    mut commands: Commands,
    handle: Res<Handle<RhaiScript>>,
    scripts: Res<Assets<RhaiScript>>,
) {
    //If the script was loaded
    if let Some(script) = scripts.get(handle.id) {
        //Create a new Engine
        let mut engine = Engine::new();
        //Register the CustomType B, and its functions we want to expose
        //(they don't have to be chained one after the other)
        engine
            .register_type::<B>()
            .register_fn("increment", B::increment);
        //Compile the script's contents
        let ast = engine.compile(&script.content).unwrap();
        //We store the resource for further use
        commands.insert_resource(ScriptingEngine(engine, ast));
    }
}
//Eveny frame this system executes the script we loaded
fn increment_in_script(
    engine_and_ast: Option<Res<ScriptingEngine>>,
    mut query: Query<(&A, &mut B)>,
) {
    //If the engine and AST were created
    if let Some(scripting_engine) = engine_and_ast {
        //Create a new scope
        let mut scope = Scope::new();
        //Iterate on our query
        for (a, mut b) in query.iter_mut() {
            //Call our scripted function and store the result
            let result: Array = scripting_engine
                .0
                .call_fn(
                    &mut scope,
                    &scripting_engine.1,
                    "do_something_with_a_and_b",
                    (a.clone(), b.clone()),
                )
                .unwrap();
            //In this case we didn't really need to send A back, but since we did, we take the second item which is B and cast it back to B
            let item = result[1].clone().cast::<B>();
            //We update B for this entity
            *b = item;
            //We print it, just to show it is indeed updating
            dbg!(b);
        }
    }
}
