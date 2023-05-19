use std::fmt::Display;



struct City {
    name:&'static str,
    lat:f32,
    lng:f32,
}


impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
        let lng_c = if self.lng >= 0.0 { "E" } else { "W" };

        write!(f, "{}: {:.3}°{} {:.3}°{} ", self.name, self.lat.abs(), lat_c, self.lng.abs(), lng_c)
    }
}

#[derive(Debug, Clone)]
struct Color {
    red:u8,
    green:u8,
    blue:u8,
}

// 实现Color的Display trait 使打印的结果类似于 RGB (128, 255, 90) 0x80FF5A
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RGB ({0},{1},{2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lng: -6.259722 },
        City { name: "Oslo", lat: 59.95, lng: 10.75 },
        City { name: "Vancouver", lat: 49.25, lng: -123.1 },
    ].iter(){
        println!("{}",city);
    }
    for color in [
        Color { red:128, green:255, blue:90 },
        Color { red:0, green:3, blue:254 },
        Color { red:0, green:0, blue:0 },
    ].iter() {
        println!("{}",color);
    }
}
