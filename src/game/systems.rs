use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if *simulation_state == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation paused");
        }
        else if *simulation_state == SimulationState::Paused{
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation continued");
        }
    }
}