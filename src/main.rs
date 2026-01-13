mod drone;
mod formation;
mod mission;
mod swarm;
mod api;

// use clap::{App, Arg, Command};
use clap::{Arg, ArgAction, Command};
use swarm::DroneSwarm;
use drone::Position;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("UAV Swarm Controller")
        .version("0.1.0")
        .about("Manages collaborative navigation for drone swarm")
        .subcommand(
            Command::new("start")
                .about("Start the swarm simulation")
        )
        .subcommand(
            Command::new("formation")
                .about("Set formation type")
                .arg(Arg::new("type")
                    .value_parser(["triangle", "line", "v_formation"])
                    .required(true))
        )
        .subcommand(
            Command::new("mission")
                .about("Execute a mission")
                .arg(Arg::new("target_x").required(true))
                .arg(Arg::new("target_y").required(true))
                .arg(Arg::new("target_z").required(true))
        )
        .subcommand(
            Command::new("serve")
                .about("Start the REST API server")
                .arg(Arg::new("host")
                    .long("host")
                    .default_value("127.0.0.1")
                    .help("Host to bind the server to"))
                .arg(Arg::new("port")
                    .long("port")
                    .short('p')
                    .default_value("8080")
                    .help("Port to bind the server to"))
        )
        .get_matches();

    let mut swarm = DroneSwarm::new();
    
    // Initialize 3 drones
    swarm.add_drone("drone_1", Position::new(0.0, 0.0, 10.0));
    swarm.add_drone("drone_2", Position::new(5.0, 0.0, 10.0));
    swarm.add_drone("drone_3", Position::new(-5.0, 0.0, 10.0));

    match matches.subcommand() {
        Some(("start", _)) => {
            println!("Starting swarm simulation...");
            swarm.start_simulation().await;
        }
        Some(("formation", sub_matches)) => {
            if let Some(formation_type) = sub_matches.get_one::<String>("type") {
                swarm.set_formation(formation_type);
                println!("Formation set to: {}", formation_type);
            }
        }
        Some(("mission", sub_matches)) => {
            let x: f64 = sub_matches.get_one::<String>("target_x").unwrap().parse()?;
            let y: f64 = sub_matches.get_one::<String>("target_y").unwrap().parse()?;
            let z: f64 = sub_matches.get_one::<String>("target_z").unwrap().parse()?;

            let target = Position::new(x, y, z);
            swarm.execute_mission(target).await;
            println!("Mission to ({}, {}, {}) completed", x, y, z);
        }
        Some(("serve", sub_matches)) => {
            let host = sub_matches.get_one::<String>("host").unwrap();
            let port: u16 = sub_matches.get_one::<String>("port").unwrap().parse()?;

            println!("Starting REST API server on {}:{}...", host, port);
            api::run_server(swarm, host, port).await?;
        }
        _ => {
            println!("Use --help for usage information");
        }
    }

    Ok(())
}
