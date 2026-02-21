mod drone;
mod formation;
mod mission;
mod swarm;
mod api;
mod simulation;
mod ports;

use clap::{Arg, Command};
use swarm::DroneSwarm;
use drone::Position;
use simulation::{SimulationConfig, SimulationMode, InternalSimulationEngine, GazeboSimulationEngine, Ros2SimulationEngine};
use simulation::{InternalCommandDispatcher, GazeboCommandDispatcher, Ros2CommandDispatcher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("UAV Swarm Controller")
        .version("0.1.0")
        .about("Manages collaborative navigation for drone swarm")
        .arg(Arg::new("mode")
            .long("mode")
            .short('m')
            .value_parser(["internal", "gazebo", "ros2"])
            .default_value("internal")
            .help("Simulation mode: internal (Rust physics) or gazebo (external simulation)")
            .global(true))
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .value_name("FILE")
            .help("Path to simulation configuration file (TOML)")
            .global(true))
        .arg(Arg::new("drones")
            .long("drones")
            .short('n')
            .value_name("COUNT")
            .default_value("3")
            .value_parser(clap::value_parser!(u32).range(1..=100))
            .help("Number of drones in the swarm (1–100)")
            .global(true))
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

    // Load simulation configuration
    let mut config = if let Some(config_path) = matches.get_one::<String>("config") {
        println!("Loading configuration from: {}", config_path);
        SimulationConfig::from_file_with_env(config_path)
            .unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load config file: {}", e);
                eprintln!("Using default configuration with environment overrides");
                SimulationConfig::from_env()
            })
    } else {
        // Try default config file, fallback to defaults
        SimulationConfig::from_file_with_env("config/simulation.toml")
            .unwrap_or_else(|_| {
                println!("No config file found, using defaults");
                SimulationConfig::from_env()
            })
    };

    // Override mode if specified via CLI argument
    if let Some(mode_str) = matches.get_one::<String>("mode") {
        if let Some(mode) = SimulationMode::from_str(mode_str) {
            config.simulation.mode = mode;
            println!("Simulation mode set to: {}", mode_str);
        }
    }

    // Create simulation engine and command dispatcher based on mode
    let (mut engine, dispatcher): (Box<dyn simulation::SimulationEngine>, Box<dyn ports::CommandDispatcher>) =
        match config.simulation.mode {
            SimulationMode::Internal => {
                println!("Using internal simulation engine (Rust physics)");
                (
                    Box::new(InternalSimulationEngine::new()),
                    Box::new(InternalCommandDispatcher),
                )
            }
            SimulationMode::Gazebo => {
                println!("Using Gazebo simulation engine");
                println!("Gazebo bridge URL: {}", config.gazebo.bridge_url);
                (
                    Box::new(GazeboSimulationEngine::new(
                        config.gazebo.bridge_url.clone(),
                        config.gazebo.timeout_ms,
                    )),
                    Box::new(GazeboCommandDispatcher::new(
                        config.gazebo.bridge_url.clone(),
                        config.gazebo.timeout_ms,
                    )),
                )
            }
            SimulationMode::Ros2 => {
                println!("Using ROS2 simulation engine");
                println!("ROS2 bridge URL: {}", config.ros2.bridge_url);
                (
                    Box::new(Ros2SimulationEngine::new(
                        config.ros2.bridge_url.clone(),
                        config.ros2.timeout_ms,
                    )),
                    Box::new(Ros2CommandDispatcher::new(
                        config.ros2.bridge_url.clone(),
                        config.ros2.timeout_ms,
                    )),
                )
            }
        };

    // Initialize the engine (dispatcher needs no initialization)
    let dispatcher = if let Err(e) = engine.initialize().await {
        eprintln!("Error: Failed to initialize simulation engine: {}", e);
        eprintln!("Falling back to internal simulation mode");
        engine = Box::new(InternalSimulationEngine::new());
        engine.initialize().await?;
        let d: Box<dyn ports::CommandDispatcher> = Box::new(InternalCommandDispatcher);
        d
    } else {
        dispatcher
    };

    // Create swarm with engine and dispatcher (composition root)
    let mut swarm = DroneSwarm::new_with_engine_and_dispatcher(engine, dispatcher);

    // Initialize drones spread along the X axis, centered on origin
    let drone_count = *matches.get_one::<u32>("drones").unwrap();
    for i in 0..drone_count {
        let x = (i as f64 - (drone_count as f64 - 1.0) / 2.0) * 5.0;
        swarm.add_drone(&format!("drone_{}", i + 1), Position::new(x, 0.0, 10.0));
    }

    println!("Swarm initialized with {} drones", swarm.drones.len());
    println!("Current simulation mode: {}", swarm.get_simulation_mode().as_str());

    match matches.subcommand() {
        Some(("start", _)) => {
            println!("Starting swarm simulation...");
            swarm.start_simulation().await;
        }
        Some(("formation", sub_matches)) => {
            if let Some(formation_type) = sub_matches.get_one::<String>("type") {
                swarm.set_formation(formation_type).await;
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
            api::run_server(swarm, config, host, port).await?;
        }
        _ => {
            println!("Use --help for usage information");
        }
    }

    Ok(())
}
