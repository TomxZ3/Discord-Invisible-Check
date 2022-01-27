mod handler;
mod settings;

use crate::settings::Settings;
use colored::control;
use colored::Colorize;
use handler::Handler;
use proctitle::set_title;
use serenity::{client::bridge::gateway::GatewayIntents, prelude::*};
use std::env;

#[tokio::main]
async fn main() {
	set_title("Dinkleberg");

	if cfg!(target_os = "windows") {
		// clear console
		std::process::Command::new("cmd")
			.arg("/c")
			.arg("cls")
			.status()
			.unwrap();

		// legacy support
		#[cfg(windows)]
		if control::set_virtual_terminal(true).is_err() {
			println!("Failed to set virtual terminal")
		};
	} else {
		// clear console
		std::process::Command::new("clear").status().unwrap();
	}

	// print logo
	println!("\n{}\n", "'########::'####:'##::: ##:'##:::'##:'##:::::::'########:'########::'########:'########:::'######:::\n ##.... ##:. ##:: ###:: ##: ##::'##:: ##::::::: ##.....:: ##.... ##: ##.....:: ##.... ##:'##... ##::\n ##:::: ##:: ##:: ####: ##: ##:'##::: ##::::::: ##::::::: ##:::: ##: ##::::::: ##:::: ##: ##:::..:::\n ##:::: ##:: ##:: ## ## ##: #####:::: ##::::::: ######::: ########:: ######::: ########:: ##::'####:\n ##:::: ##:: ##:: ##. ####: ##. ##::: ##::::::: ##...:::: ##.... ##: ##...:::: ##.. ##::: ##::: ##::\n ##:::: ##:: ##:: ##:. ###: ##:. ##:: ##::::::: ##::::::: ##:::: ##: ##::::::: ##::. ##:: ##::: ##::\n ########::'####: ##::. ##: ##::. ##: ########: ########: ########:: ########: ##:::. ##:. ######:::\n........:::....::..::::..::..::::..::........::........::........:::........::..:::::..:::......::::".green());

	// get discord token
	let token = match env::var("DISCORD_TOKEN") {
		Ok(token) => token,
		Err(_) => {
			println!(
				"{}",
				"Please set the environment variable DISCORD_TOKEN to continue".red()
			);
			return;
		}
	};

	// print usage
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("{}", format!("Usage: {} <guild_id>", args[0]).red());
		return;
	}

	// parse guild
	let guild_id = match (&args[1]).parse::<u64>() {
		Ok(n) => n,
		Err(_) => panic!("{}", "Could not parse guild id".red()),
	};

	let settings = match Settings::load() {
		Ok(settings) => settings,
		Err(_) => {
			println!("{}", "Could not load settings".red());
			return;
		}
	};

	// create client with custom handler implementing trait `EventHandler`
	let mut client = Client::builder(token)
		.event_handler(Handler::new(guild_id, &settings))
		.intents(GatewayIntents::GUILD_PRESENCES | GatewayIntents::GUILD_MEMBERS)
		.await
		.unwrap_or_else(|_| panic!("{}", "Error creating client".red().to_string()));

	// start client or panic
	if let Err(why) = client.start().await {
		panic!("{}", format!("Client error: {:?}", why).red());
	}
}
