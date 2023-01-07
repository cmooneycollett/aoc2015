/// Models a reindeer as described in the AOC 2015 Day 14 problem.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Reindeer {
    speed: u64,           // km/s
    duration_travel: u64, // s
    duration_rest: u64,   // s
    is_travelling: bool,
    seconds_phase: u64,      // s
    distance_travelled: u64, // km
}

impl Reindeer {
    pub fn new(speed: u64, duration_travel: u64, duration_rest: u64) -> Reindeer {
        Reindeer {
            speed,
            duration_travel,
            duration_rest,
            is_travelling: true,
            seconds_phase: 0,
            distance_travelled: 0,
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

    /// Advances the reindeer by one second and returns the total distance is has travelled.
    pub fn advance_one_second(&mut self) -> u64 {
        if self.is_travelling {
            self.seconds_phase += 1;
            self.distance_travelled += self.speed;
            if self.seconds_phase == self.duration_travel {
                self.seconds_phase = 0;
                self.is_travelling = false;
            }
            return self.distance_travelled;
        }
        // Reindeer is at rest
        self.seconds_phase += 1;
        if self.seconds_phase == self.duration_rest {
            self.seconds_phase = 0;
            self.is_travelling = true;
        }
        self.distance_travelled
    }
}
