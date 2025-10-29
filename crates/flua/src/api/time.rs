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

// Implementation for the Stopwatch
impl Stopwatch {
    // Function to create a new Stopwatch
    fn new() -> Self {
        Stopwatch {
            start: None,
            elapsed: Duration::ZERO,
            paused: false,
        }
    }

    // Function to start the stopwatch
    fn start(&mut self) {
        if self.start.is_none() && !self.paused {
            self.start = Some(Instant::now());
        } else if self.paused {
            self.start = Some(Instant::now());
            self.paused = false;
        }
    }

    // Function to pause the stopwatch
    fn pause(&mut self) {
        if let Some(start_time) = self.start {
            if !self.paused {
                self.elapsed += start_time.elapsed();
                self.start = None;
                self.paused = true;
            }
        }
    }

    // Function to stop the stopwach
    fn stop(&mut self) {
        self.start = None;
        self.elapsed = Duration::ZERO;
        self.paused = false;
    }

    // Function to read the stopwatch
    fn read(&self) -> f64 {
        if let Some(start_time) = self.start
            && !self.paused
        {
            return (self.elapsed + start_time.elapsed()).as_secs_f64();
        }
        self.elapsed.as_secs_f64()
    }
}

// Function to register the stopwatch for Lua
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

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;
    use std::thread;
    use std::time::Duration;

    // TODO
    // Use MOCK Clock for test timings because they are manipulated by CPU load or VMs etc

    #[test]
    fn new_stopwatch_initial_state() {
        let sw = Stopwatch::new();
        assert!(sw.start.is_none());
        assert_eq!(sw.elapsed, Duration::ZERO);
        assert!(!sw.paused);
    }

    #[test]
    fn start_and_read_increases_over_time() {
        let mut sw = Stopwatch::new();
        sw.start();
        thread::sleep(Duration::from_millis(100));
        let elapsed = sw.read();
        // assert!(elapsed >= 0.09 && elapsed <= 0.2, "elapsed = {}", elapsed);
        assert!(elapsed >= 0.09, "elapsed = {}", elapsed);
    }

    #[test]
    fn pause_stops_increasing_time() {
        let mut sw = Stopwatch::new();
        sw.start();
        thread::sleep(Duration::from_millis(100));
        sw.pause();
        let before = sw.read();
        thread::sleep(Duration::from_millis(100));
        let after = sw.read();
        assert!((after - before).abs() < 0.01, "paused should not increase");
    }

    #[test]
    fn stop_resets_stopwatch() {
        let mut sw = Stopwatch::new();
        sw.start();
        thread::sleep(Duration::from_millis(50));
        sw.stop();
        assert_eq!(sw.elapsed, Duration::ZERO);
        assert!(sw.start.is_none());
        assert!(!sw.paused);
        assert_eq!(sw.read(), 0.0);
    }

    #[test]
    fn start_after_pause_continues_timing() {
        let mut sw = Stopwatch::new();
        sw.start();
        thread::sleep(Duration::from_millis(50));
        sw.pause();
        let paused_time = sw.read();
        thread::sleep(Duration::from_millis(50));
        sw.start();
        thread::sleep(Duration::from_millis(50));
        let total = sw.read();
        assert!(total > paused_time, "should continue counting after resume");
    }

    #[test]
    fn lua_can_create_and_read_stopwatch() -> Result<()> {
        let lua = Lua::new();
        let stopwatch_lib = register(&lua)?;
        lua.globals().set("stopwatch", stopwatch_lib)?;

        lua.load(
            r#"
            stopwatch.new_stopwatch("test1")
            stopwatch.start("test1")
        "#,
        )
        .exec()?;

        thread::sleep(Duration::from_millis(50));

        let elapsed: f64 = lua.load(r#"return stopwatch.read("test1")"#).eval()?;
        assert!(elapsed > 0.0);
        Ok(())
    }

    #[test]
    fn lua_pause_and_resume_works() -> Result<()> {
        let lua = Lua::new();
        let stopwatch_lib = register(&lua)?;
        lua.globals().set("stopwatch", stopwatch_lib)?;

        lua.load(
            r#"
            stopwatch.new_stopwatch("t1")
            stopwatch.start("t1")
        "#,
        )
        .exec()?;

        thread::sleep(Duration::from_millis(50));

        lua.load(r#"stopwatch.pause("t1")"#).exec()?;
        let paused_time: f64 = lua.load(r#"return stopwatch.read("t1")"#).eval()?;
        thread::sleep(Duration::from_millis(50));
        let still_same: f64 = lua.load(r#"return stopwatch.read("t1")"#).eval()?;

        assert!(
            (still_same - paused_time).abs() < 0.01,
            "Time should not increase while paused"
        );

        lua.load(r#"stopwatch.start("t1")"#).exec()?;
        thread::sleep(Duration::from_millis(50));
        let resumed_time: f64 = lua.load(r#"return stopwatch.read("t1")"#).eval()?;
        assert!(resumed_time > paused_time);

        Ok(())
    }

    #[test]
    fn lua_stop_resets() -> Result<()> {
        let lua = Lua::new();
        let stopwatch_lib = register(&lua)?;
        lua.globals().set("stopwatch", stopwatch_lib)?;

        lua.load(
            r#"
            stopwatch.new_stopwatch("t2")
            stopwatch.start("t2")
        "#,
        )
        .exec()?;
        thread::sleep(Duration::from_millis(30));

        lua.load(r#"stopwatch.stop("t2")"#).exec()?;
        let val: f64 = lua.load(r#"return stopwatch.read("t2")"#).eval()?;
        assert!(val < 0.01);
        Ok(())
    }

    #[test]
    fn lua_multiple_stopwatches_independent() -> Result<()> {
        let lua = Lua::new();
        let stopwatch_lib = register(&lua)?;
        lua.globals().set("stopwatch", stopwatch_lib)?;

        lua.load(
            r#"
            stopwatch.new_stopwatch("a")
            stopwatch.new_stopwatch("b")
            stopwatch.start("a")
        "#,
        )
        .exec()?;

        thread::sleep(Duration::from_millis(50));

        lua.load(r#"stopwatch.start("b")"#).exec()?;
        thread::sleep(Duration::from_millis(50));

        let a_time: f64 = lua.load(r#"return stopwatch.read("a")"#).eval()?;
        let b_time: f64 = lua.load(r#"return stopwatch.read("b")"#).eval()?;
        assert!(
            a_time > b_time,
            "First stopwatch should have higher elapsed time"
        );

        Ok(())
    }

    #[test]
    fn lua_wait_function_sleeps() -> Result<()> {
        let lua = Lua::new();
        let stopwatch_lib = register(&lua)?;
        lua.globals().set("stopwatch", stopwatch_lib)?;

        let start = std::time::Instant::now();
        lua.load(r#"stopwatch.wait(100)"#).exec()?;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(90));
        Ok(())
    }

    #[test]
    #[ignore]
    fn lua_wait_forever_blocks() -> Result<()> {
        // Dieser Test wird absichtlich ignoriert, da er endlos blockiert.
        // Nur zu Testzwecken zeigen, dass waitfr() existiert.
        let lua = Lua::new();
        let stopwatch_lib = register(&lua)?;
        lua.globals().set("stopwatch", stopwatch_lib)?;
        assert!(
            lua.globals()
                .get::<mlua::Function>("stopwatch.waitfr")
                .is_ok()
        );
        Ok(())
    }
}
