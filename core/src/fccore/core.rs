#![allow(dead_code)]

use fccore::config::Config;
use std::thread::sleep_ms;
use simplelog::Log;
use prismatikapi::*;

use time;

const TAG : &'static str = "core";
const LOG_DIR : &'static str = "./logs/";

pub struct Core {

    /**
     * Is the core alive
     */
    pub alive: bool,

    pub on: bool,

    prismatik: Prismatik,
  
    /**
     * configuration for the core
     */
    config: Config,
  
    /**
     * Core log, stores log messages and timestamps
     */
    log: Log
}

impl Core {

    pub fn new(config_file : &str) -> Core {
        let config = Config::load(config_file);
        let mut core = Core {
            alive: true,
            on: false,
            prismatik: Prismatik::new(&config.server_url, &config.api_key),
            log: Log::new(&format!("{}log{}", LOG_DIR, time::now().to_timespec().sec), config.log_config.log_limit),
            config: config
        };
        core.log.add(TAG, &format!("Connecting to server {} with key {}", &core.config.server_url, &core.config.api_key));
        core.prismatik.set_brightness(75);
        core.prismatik.set_smooth(150);
        core.prismatik.set_all_lights(200, 0, 0);
        core.set_on(false);
        core
    }

    pub fn set_on(&mut self, on: bool) {
        self.on = on;
        self.prismatik.set_on(on);
        self.log.add(TAG, &("Set ON to ".to_string() + &on.to_string()));
    }

    pub fn set_color_all(&mut self, r: usize, g: usize, b: usize) {
        self.prismatik.set_all_lights(r,g,b);
        self.log.add(TAG, &("Set color to ".to_string() + &r.to_string() + ", " + &g.to_string() + ", " + &b.to_string()));
    }

    pub fn update(&mut self) {}

    /**
     * Get the core config struct
     */
    pub fn config(&self) -> &Config { &self.config }
    
    /**
     * Return the core log
     */
    pub fn log(&self) -> &Log { &self.log }
    
    /**
     * Return the core log as mutable
     */
     pub fn log_mut(&mut self) -> &mut Log { &mut self.log }
}
