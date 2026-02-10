mod drone;
mod formation;
mod mission;
mod swarm;
mod api;
mod simulation;

use clap::{Arg, Command};
use swarm::DroneSwarm;
use drone::Position;
use simulation::{SimulationConfig, SimulationMode, InternalSimulationEngine, GazeboSimulationEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("UAV Swarm Controller")
        .version("0.1.0")
        .about("Manages collaborative navigation for drone swarm")
        .arg(Arg::new("mode")
            .long("mode")
            .short('m')
            .value_parser(["internal", "gazebo"])
            .default_value("internal")
            .help("Simulation mode: internal (Rust physics) or gazebo (external simulation)")
            .global(true))
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .value_name("FILE")
            .help("Path to simulation configuration file (TOML)")
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

    // Create simulation engine based on mode
    let mut engine: Box<dyn simulation::SimulationEngine> = match config.simulation.mode {
        SimulationMode::Internal => {
            println!("Using internal simulation engine (Rust physics)");
            Box::new(InternalSimulationEngine::new())
        }
        SimulationMode::Gazebo => {
            println!("Using Gazebo simulation engine");
            println!("Gazebo bridge URL: {}", config.gazebo.bridge_url);
            Box::new(GazeboSimulationEngine::new(
                config.gazebo.bridge_url.clone(),
                config.gazebo.timeout_ms,
            ))
        }
    };

    // Initialize the engine
    if let Err(e) = engine.initialize().await {
        eprintln!("Error: Failed to initialize simulation engine: {}", e);
        eprintln!("Falling back to internal simulation mode");
        engine = Box::new(InternalSimulationEngine::new());
        engine.initialize().await?;
    }

    // Create swarm with the configured engine
    let mut swarm = DroneSwarm::new_with_engine(engine);

    // Initialize 3 drones
    swarm.add_drone("drone_1", Position::new(0.0, 0.0, 10.0));
    swarm.add_drone("drone_2", Position::new(5.0, 0.0, 10.0));
    swarm.add_drone("drone_3", Position::new(-5.0, 0.0, 10.0));

    println!("Swarm initialized with {} drones", swarm.drones.len());
    println!("Current simulation mode: {}", swarm.get_simulation_mode().as_str());

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
            api::run_server(swarm, config, host, port).await?;
        }
        _ => {
            println!("Use --help for usage information");
        }
    }

    Ok(())
}
