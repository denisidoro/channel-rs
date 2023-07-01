use super::payload::Command::{self, *};
use super::terminal;
// use super::{ble, Device};
use crate::prelude::*;
use crate::utils::color::hsl_to_rgb;
use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct Input {
    commands: String,
    // mac: String,
    // peripheral: Uuid,
    // service: Uuid,
    // characteristic: Uuid,
    // #[clap(short, long)]
    // print: bool,
}

impl Runnable for Input {
    fn run(&self, _system: System) -> Result<()> {
        let command_vec = string_to_commands(&self.commands);
        // let device = Device {
        //     mac: self.mac.clone(),
        //     peripheral: self.peripheral,
        //     service: self.service,
        //     characteristic: self.characteristic,
        // };
        // if self.print {
        terminal::main(command_vec)?;
        // } else {
        //     executor::block_on(ble::main(command_vec, device))?;
        // }
        Ok(())
    }
}

impl HasDeps for Input {}

fn string_to_command(txt: &str) -> Result<Command> {
    match txt {
        "on" => Ok(Power(true)),
        "off" => Ok(Power(false)),
        "maroon" => Ok(Rgb(128, 0, 0)),
        "dark red" => Ok(Rgb(139, 0, 0)),
        "brown" => Ok(Rgb(165, 42, 42)),
        "firebrick" => Ok(Rgb(178, 34, 34)),
        "crimson" => Ok(Rgb(220, 20, 60)),
        "red" => Ok(Rgb(255, 0, 0)),
        "tomato" => Ok(Rgb(255, 99, 71)),
        "coral" => Ok(Rgb(255, 127, 80)),
        "indian red" => Ok(Rgb(205, 92, 92)),
        "light coral" => Ok(Rgb(240, 128, 128)),
        "dark salmon" => Ok(Rgb(233, 150, 122)),
        "salmon" => Ok(Rgb(250, 128, 114)),
        "light salmon" => Ok(Rgb(255, 160, 122)),
        "orange red" => Ok(Rgb(255, 69, 0)),
        "dark orange" => Ok(Rgb(255, 140, 0)),
        "orange" => Ok(Rgb(255, 165, 0)),
        "gold" => Ok(Rgb(255, 215, 0)),
        "dark golden rod" => Ok(Rgb(184, 134, 11)),
        "golden rod" => Ok(Rgb(218, 165, 32)),
        "pale golden rod" => Ok(Rgb(238, 232, 170)),
        "dark khaki" => Ok(Rgb(189, 183, 107)),
        "khaki" => Ok(Rgb(240, 230, 140)),
        "olive" => Ok(Rgb(128, 128, 0)),
        "yellow" => Ok(Rgb(255, 255, 0)),
        "yellow green" => Ok(Rgb(154, 205, 50)),
        "dark olive green" => Ok(Rgb(85, 107, 47)),
        "olive drab" => Ok(Rgb(107, 142, 35)),
        "lawn green" => Ok(Rgb(124, 252, 0)),
        "chartreuse" => Ok(Rgb(127, 255, 0)),
        "green yellow" => Ok(Rgb(173, 255, 47)),
        "dark green" => Ok(Rgb(0, 100, 0)),
        "green" => Ok(Rgb(0, 128, 0)),
        "forest green" => Ok(Rgb(34, 139, 34)),
        "lime" => Ok(Rgb(0, 255, 0)),
        "lime green" => Ok(Rgb(50, 205, 50)),
        "light green" => Ok(Rgb(144, 238, 144)),
        "pale green" => Ok(Rgb(152, 251, 152)),
        "dark sea green" => Ok(Rgb(143, 188, 143)),
        "medium spring green" => Ok(Rgb(0, 250, 154)),
        "spring green" => Ok(Rgb(0, 255, 127)),
        "sea green" => Ok(Rgb(46, 139, 87)),
        "medium aqua marine" => Ok(Rgb(102, 205, 170)),
        "medium sea green" => Ok(Rgb(60, 179, 113)),
        "light sea green" => Ok(Rgb(32, 178, 170)),
        "dark slate gray" => Ok(Rgb(47, 79, 79)),
        "teal" => Ok(Rgb(0, 128, 128)),
        "dark cyan" => Ok(Rgb(0, 139, 139)),
        "aqua" => Ok(Rgb(0, 255, 255)),
        "cyan" => Ok(Rgb(0, 255, 255)),
        "light cyan" => Ok(Rgb(224, 255, 255)),
        "dark turquoise" => Ok(Rgb(0, 206, 209)),
        "turquoise" => Ok(Rgb(64, 224, 208)),
        "medium turquoise" => Ok(Rgb(72, 209, 204)),
        "pale turquoise" => Ok(Rgb(175, 238, 238)),
        "aqua marine" => Ok(Rgb(127, 255, 212)),
        "powder blue" => Ok(Rgb(176, 224, 230)),
        "cadet blue" => Ok(Rgb(95, 158, 160)),
        "steel blue" => Ok(Rgb(70, 130, 180)),
        "corn flower blue" => Ok(Rgb(100, 149, 237)),
        "deep sky blue" => Ok(Rgb(0, 191, 255)),
        "dodger blue" => Ok(Rgb(30, 144, 255)),
        "light blue" => Ok(Rgb(173, 216, 230)),
        "sky blue" => Ok(Rgb(135, 206, 235)),
        "light sky blue" => Ok(Rgb(135, 206, 250)),
        "midnight blue" => Ok(Rgb(25, 25, 112)),
        "navy" => Ok(Rgb(0, 0, 128)),
        "dark blue" => Ok(Rgb(0, 0, 139)),
        "medium blue" => Ok(Rgb(0, 0, 205)),
        "blue" => Ok(Rgb(0, 0, 255)),
        "royal blue" => Ok(Rgb(65, 105, 225)),
        "blue violet" => Ok(Rgb(138, 43, 226)),
        "indigo" => Ok(Rgb(75, 0, 130)),
        "dark slate blue" => Ok(Rgb(72, 61, 139)),
        "slate blue" => Ok(Rgb(106, 90, 205)),
        "medium slate blue" => Ok(Rgb(123, 104, 238)),
        "medium purple" => Ok(Rgb(147, 112, 219)),
        "dark magenta" => Ok(Rgb(139, 0, 139)),
        "dark violet" => Ok(Rgb(148, 0, 211)),
        "dark orchid" => Ok(Rgb(153, 50, 204)),
        "medium orchid" => Ok(Rgb(186, 85, 211)),
        "purple" => Ok(Rgb(128, 0, 128)),
        "thistle" => Ok(Rgb(216, 191, 216)),
        "plum" => Ok(Rgb(221, 160, 221)),
        "violet" => Ok(Rgb(238, 130, 238)),
        "magenta" => Ok(Rgb(255, 0, 255)),
        "orchid" => Ok(Rgb(218, 112, 214)),
        "medium violet red" => Ok(Rgb(199, 21, 133)),
        "pale violet red" => Ok(Rgb(219, 112, 147)),
        "deep pink" => Ok(Rgb(255, 20, 147)),
        "hot pink" => Ok(Rgb(255, 105, 180)),
        "light pink" => Ok(Rgb(255, 182, 193)),
        "pink" => Ok(Rgb(255, 192, 203)),
        "antique white" => Ok(Rgb(250, 235, 215)),
        "beige" => Ok(Rgb(245, 245, 220)),
        "bisque" => Ok(Rgb(255, 228, 196)),
        "blanched almond" => Ok(Rgb(255, 235, 205)),
        "wheat" => Ok(Rgb(245, 222, 179)),
        "corn silk" => Ok(Rgb(255, 248, 220)),
        "lemon chiffon" => Ok(Rgb(255, 250, 205)),
        "light golden rod yellow" => Ok(Rgb(250, 250, 210)),
        "light yellow" => Ok(Rgb(255, 255, 224)),
        "saddle brown" => Ok(Rgb(139, 69, 19)),
        "sienna" => Ok(Rgb(160, 82, 45)),
        "chocolate" => Ok(Rgb(210, 105, 30)),
        "peru" => Ok(Rgb(205, 133, 63)),
        "sandy brown" => Ok(Rgb(244, 164, 96)),
        "burly wood" => Ok(Rgb(222, 184, 135)),
        "tan" => Ok(Rgb(210, 180, 140)),
        "rosy brown" => Ok(Rgb(188, 143, 143)),
        "moccasin" => Ok(Rgb(255, 228, 181)),
        "navajo white" => Ok(Rgb(255, 222, 173)),
        "peach puff" => Ok(Rgb(255, 218, 185)),
        "misty rose" => Ok(Rgb(255, 228, 225)),
        "lavender blush" => Ok(Rgb(255, 240, 245)),
        "linen" => Ok(Rgb(250, 240, 230)),
        "old lace" => Ok(Rgb(253, 245, 230)),
        "papaya whip" => Ok(Rgb(255, 239, 213)),
        "sea shell" => Ok(Rgb(255, 245, 238)),
        "mint cream" => Ok(Rgb(245, 255, 250)),
        "slate gray" => Ok(Rgb(112, 128, 144)),
        "light slate gray" => Ok(Rgb(119, 136, 153)),
        "light steel blue" => Ok(Rgb(176, 196, 222)),
        "lavender" => Ok(Rgb(230, 230, 250)),
        "floral white" => Ok(Rgb(255, 250, 240)),
        "alice blue" => Ok(Rgb(240, 248, 255)),
        "ghost white" => Ok(Rgb(248, 248, 255)),
        "honeydew" => Ok(Rgb(240, 255, 240)),
        "ivory" => Ok(Rgb(255, 255, 240)),
        "azure" => Ok(Rgb(240, 255, 255)),
        "snow" => Ok(Rgb(255, 250, 250)),
        "black" => Ok(Rgb(0, 0, 0)),
        "dim gray" => Ok(Rgb(105, 105, 105)),
        "gray" => Ok(Rgb(128, 128, 128)),
        "dark gray" => Ok(Rgb(169, 169, 169)),
        "silver" => Ok(Rgb(192, 192, 192)),
        "light gray" => Ok(Rgb(211, 211, 211)),
        "gainsboro" => Ok(Rgb(220, 220, 220)),
        "white smoke" => Ok(Rgb(245, 245, 245)),
        "white" => Ok(Rgb(255, 255, 255)),
        _ => {
            if txt.starts_with('#') {
                let r = u8::from_str_radix(&txt[1..3], 16)?;
                let g = u8::from_str_radix(&txt[3..5], 16)?;
                let b = u8::from_str_radix(&txt[5..7], 16)?;
                Ok(Rgb(r, g, b))
            } else if txt.starts_with("rgb(") {
                let mut parts = txt[4..txt.len() - 1].split(',');
                let r: u8 = parts.next().context("no R part")?.trim().parse()?;
                let g: u8 = parts.next().context("no G part")?.trim().parse()?;
                let b: u8 = parts.next().context("no B part")?.trim().parse()?;
                Ok(Rgb(r, g, b))
            } else if txt.starts_with("hsl(") {
                let mut parts = txt[4..txt.len() - 1].split(',');
                let h: f32 = parts.next().context("no H part")?.trim().parse()?;
                let s: f32 = parts.next().context("no S part")?.trim().parse()?;
                let l: f32 = parts.next().context("no L part")?.trim().parse()?;
                let (r, g, b) = hsl_to_rgb(h, s, l);
                Ok(Rgb(r, g, b))
            } else if txt.starts_with('b') {
                let b: u8 = txt[1..txt.len()].parse()?;
                Ok(Brightness(b))
            } else if txt.starts_with('w') {
                let w: u16 = txt[1..txt.len()].parse()?;
                Ok(Wait(w))
            } else {
                Err(anyhow!("invalid command: {txt}"))
            }
        }
    }
}

// TODO: ignore invalid commands?
fn string_to_commands(txt: &str) -> Vec<Command> {
    txt.split(',')
        .map(|t| string_to_command(t).expect("invalid command"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload() {
        let cases = [
            ("#AA01FF", Rgb(170, 1, 255)),
            ("rgb(170 ,1, 255)", Rgb(170, 1, 255)),
            ("hsl(325, 50.9, 44.7)", Rgb(172, 55, 123)),
            ("b32", Brightness(32)),
        ];
        for (input, expected) in cases {
            let out = string_to_command(input).unwrap();
            assert_eq!(out, expected);
        }
    }
}
