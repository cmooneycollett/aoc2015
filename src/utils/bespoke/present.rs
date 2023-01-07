/// Represents a present box (rectangular prism) characterised by its length, width and height
/// (measures in feet). Used in the AOC 2015 Day 2
pub struct Present {
    length: u64,
    width: u64,
    height: u64,
}

impl Present {
    pub fn new(length: u64, width: u64, height: u64) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    /// Calculates the amount of wrapping paper needed to wrap the present (measured in square
    /// feet). Result is the present surface area plus the area of the smallest side.
    pub fn paper_needed(&self) -> u64 {
        let side_areas: [u64; 3] = [
            self.length * self.width,
            self.length * self.height,
            self.width * self.height,
        ];
        2 * side_areas.iter().sum::<u64>() + side_areas.iter().min().unwrap()
    }

    /// Calculates the amount of ribbon needed to wrap the present (measured in feet). Result is
    /// the smallest side perimeter plus the present volume mapped from cubic feet to linear feet.
    pub fn ribbon_needed(&self) -> u64 {
        let side_perims: [u64; 3] = [
            2 * (self.length + self.width),
            2 * (self.length + self.height),
            2 * (self.width + self.height),
        ];
        side_perims.iter().min().unwrap() + self.length * self.width * self.height
    }
}
