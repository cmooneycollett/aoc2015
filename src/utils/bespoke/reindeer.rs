/// Models a reindeer as described in the AOC 2015 Day 14 problem.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Reindeer {
    speed: u64,           // km/s
    duration_travel: u64, // s
    duration_rest: u64,   // s
}

impl Reindeer {
    pub fn new(speed: u64, duration_travel: u64, duration_rest: u64) -> Reindeer {
        Reindeer {
            speed,
            duration_travel,
            duration_rest,
        }
    }

    /// Determines the distance travelled by the reindeer during the specified race duration.
    pub fn distance_travelled_in_period(&self, duration: u64) -> u64 {
        // Calculate number of completed cycles and spare seconds
        let cycle_period = self.duration_travel + self.duration_rest;
        let cycles_complete = duration / cycle_period;
        let seconds_spare = duration % cycle_period;
        // Calculate distance travelled
        let mut distance_travelled = cycles_complete * self.speed * self.duration_travel;
        if seconds_spare >= self.duration_travel {
            distance_travelled += self.speed * self.duration_travel;
        } else {
            distance_travelled += self.speed * seconds_spare;
        }
        distance_travelled
    }
}
