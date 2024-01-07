use bevy::{
    ecs::system::{Res, ResMut, Resource},
    time::Time,
};

#[derive(Resource, Clone, Debug, PartialEq, PartialOrd)]
pub struct SimulationTime(f64);

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimeScale {
    Paused,
    Normal,
    Fast,
    Fastest,
}

const MINUTES_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const TICKS_PER_DAY: u64 = MINUTES_PER_HOUR * HOURS_PER_DAY;
const DAYS_PER_YEAR: u64 = 360;
const DEFAULT_TICK: u64 = 720;

impl Default for SimulationTime {
    fn default() -> Self {
        SimulationTime(DEFAULT_TICK as f64)
    }
}

impl SimulationTime {
    // Converts from fractional seconds to ticks
    pub fn ticks(&self) -> u64 {
        self.0.floor() as u64
    }

    pub fn is_first_tick(&self) -> bool {
        self.ticks() == 0
    }

    pub fn get_year(&self) -> u64 {
        self.ticks() / TICKS_PER_DAY / DAYS_PER_YEAR
    }

    pub fn get_day(&self) -> u64 {
        (self.ticks() / TICKS_PER_DAY) % DAYS_PER_YEAR
    }

    pub fn get_time(&self) -> u64 {
        self.ticks() % TICKS_PER_DAY
    }

    pub fn get_hour_minute(&self) -> (u32, u32) {
        let time = self.get_time();
        let hour = time / MINUTES_PER_HOUR;
        let minute = time % MINUTES_PER_HOUR;
        (hour as u32, minute as u32)
    }

    pub fn update(&mut self, delta_seconds: f64) {
        self.0 += delta_seconds;
    }

    pub fn time_since_ticks(&self, other: &SimulationTime) -> u64 {
        assert!(self.0 >= other.0);

        SimulationTime(self.0 - other.0).ticks()
    }
}

impl std::fmt::Display for SimulationTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hour, minute) = self.get_hour_minute();
        write!(
            f,
            "Year {}, Day {}, {:02}:{:02}",
            self.get_year(),
            self.get_day(),
            hour,
            minute
        )
    }
}

#[derive(Resource, Clone, Debug)]
pub struct SimulationDeltaTime(pub Option<f64>);

impl SimulationDeltaTime {
    fn update(&mut self, time_scale: &TimeScale, real_time: &Time) {
        self.0 = match time_scale {
            TimeScale::Paused => None,
            TimeScale::Normal => Some(real_time.delta_seconds_f64()),
            TimeScale::Fast => Some(real_time.delta_seconds_f64() * 2.0),
            TimeScale::Fastest => Some(real_time.delta_seconds_f64() * 4.0),
        };
    }
}

pub fn update_simulation_time(
    time_scale: Res<TimeScale>,
    time: Res<Time>,
    mut simulation_delta_time: ResMut<SimulationDeltaTime>,
) {
    simulation_delta_time.update(&time_scale, &time);
}

pub fn advance_time(
    simulation_time: Res<SimulationDeltaTime>,
    mut world_time: ResMut<SimulationTime>,
) {
    let Some(simulation_delta_time) = simulation_time.0 else {
        return;
    };

    world_time.update(simulation_delta_time);
}
