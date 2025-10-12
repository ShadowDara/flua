use mlua::{Lua, Result, Table};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Stopwatch {
    start: Option<Instant>,
    elapsed: Duration,
    paused: bool,
}

impl Stopwatch {
    fn new() -> Self {
        Stopwatch {
            start: None,
            elapsed: Duration::ZERO,
            paused: false,
        }
    }

    fn start(&mut self) {
        if self.start.is_none() && !self.paused {
            self.start = Some(Instant::now());
        } else if self.paused {
            self.start = Some(Instant::now());
            self.paused = false;
        }
    }

    fn pause(&mut self) {
        if let Some(start_time) = self.start {
            if !self.paused {
                self.elapsed += start_time.elapsed();
                self.start = None;
                self.paused = true;
            }
        }
    }

    fn stop(&mut self) {
        self.start = None;
        self.elapsed = Duration::ZERO;
        self.paused = false;
    }

    fn read(&self) -> f64 {
        if let Some(start_time) = self.start {
            if !self.paused {
                return (self.elapsed + start_time.elapsed()).as_secs_f64();
            }
        }
        self.elapsed.as_secs_f64()
    }
}

pub fn register(lua: &Lua) -> Result<Table> {
    // HashMap zur Verwaltung mehrerer Stoppuhren per ID
    let watches: Arc<Mutex<HashMap<String, Stopwatch>>> = Arc::new(Mutex::new(HashMap::new()));
    let table = lua.create_table()?;

    // new_stopwatch(id)
    {
        let watches = Arc::clone(&watches);
        table.set(
            "new_stopwatch",
            lua.create_function(move |_, id: String| {
                let mut map = watches.lock().unwrap();
                map.insert(id.clone(), Stopwatch::new());
                println!("Created stopwatch '{}'", id);
                Ok(())
            })?,
        )?;
    }

    // start(id)
    {
        let watches = Arc::clone(&watches);
        table.set(
            "start",
            lua.create_function(move |_, id: String| {
                let mut map = watches.lock().unwrap();
                if let Some(sw) = map.get_mut(&id) {
                    sw.start();
                    println!("Started stopwatch '{}'", id);
                } else {
                    println!("No stopwatch found for '{}'", id);
                }
                Ok(())
            })?,
        )?;
    }

    // pause(id)
    {
        let watches = Arc::clone(&watches);
        table.set(
            "pause",
            lua.create_function(move |_, id: String| {
                let mut map = watches.lock().unwrap();
                if let Some(sw) = map.get_mut(&id) {
                    sw.pause();
                    println!("Paused stopwatch '{}'", id);
                } else {
                    println!("No stopwatch found for '{}'", id);
                }
                Ok(())
            })?,
        )?;
    }

    // stop(id)
    {
        let watches = Arc::clone(&watches);
        table.set(
            "stop",
            lua.create_function(move |_, id: String| {
                let mut map = watches.lock().unwrap();
                if let Some(sw) = map.get_mut(&id) {
                    sw.stop();
                    println!("Stopped and reset stopwatch '{}'", id);
                } else {
                    println!("No stopwatch found for '{}'", id);
                }
                Ok(())
            })?,
        )?;
    }

    // read(id) -> float seconds
    {
        let watches = Arc::clone(&watches);
        table.set(
            "read",
            lua.create_function(move |_, id: String| {
                let map = watches.lock().unwrap();
                if let Some(sw) = map.get(&id) {
                    Ok(sw.read())
                } else {
                    println!("No stopwatch found for '{}'", id);
                    Ok(0.0)
                }
            })?,
        )?;
    }

    // Function to pause the programm for a certain amount of time
    // Does not work with negative Numbers!
    let wait = lua.create_function(|_, time: u64| {
        thread::sleep(Duration::from_millis(time));
        Ok(())
    })?;

    // function to wait forever, so a startet http does not shutdown at the end of the script
    let wait_forever = lua.create_function(|_, ()| {
        // Endlosschleife, blockiert den Thread für immer
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
        #[allow(unreachable_code)]
        Ok(())
    })?;

    table.set("wait", wait)?;
    table.set("waitfr", wait_forever)?;

    Ok(table)
}
