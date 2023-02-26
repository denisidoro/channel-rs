#[derive(Clone, Debug)] // TODO: remove clone
pub enum Command {
    Brightness(u8),
    Speed(u8),
    TemperatureMode,
    Effect(u8),
    Power(bool),
    Grayscale(u8),
    Temperature(u8),
    Rgb(u8, u8, u8),
    Wait(u16),
}

pub type Payload = [u8; 9];

impl From<Command> for Payload {
    fn from(command: Command) -> Self {
        let mut payload = [0x7e, 0, 0, 0, 0, 0, 0, 0, 0xef];

        use Command::*;
        let slice = match command {
            Brightness(v) => [1, v, 0, 0, 0],
            Power(v) => [4, u8::from(v), 0, 0, 0],
            Rgb(r, g, b) => [5, 3, r, g, b],
            _ => unimplemented!("command not implemented"),
        };

        payload[2] = slice[0];
        payload[3] = slice[1];
        payload[4] = slice[2];
        payload[5] = slice[3];
        payload[6] = slice[4];

        payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Command::*;

    #[test]
    fn test_payload() {
        let cases = [
            (
                Brightness(0),
                [0x7e, 0x00, 0x01, 0x0, 0x00, 0x00, 0x00, 0x00, 0xef],
            ),
            (
                Brightness(50),
                [0x7e, 0x00, 0x01, 0x32, 0x00, 0x00, 0x00, 0x00, 0xef],
            ),
            (
                Brightness(100),
                [0x7e, 0x00, 0x01, 0x64, 0x00, 0x00, 0x00, 0x00, 0xef],
            ),
            (
                Power(false),
                [0x7e, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef],
            ),
            (
                Power(true),
                [0x7e, 0x00, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0xef],
            ),
            (
                Rgb(255, 255, 255),
                [0x7e, 0x00, 0x05, 0x03, 0xff, 0xff, 0xff, 0x00, 0xef],
            ),
            (
                Rgb(124, 125, 126),
                [0x7e, 0x00, 0x05, 0x03, 0x7c, 0x7d, 0x7e, 0x00, 0xef],
            ),
        ];
        for (input, expected) in cases {
            let out: Payload = input.into();
            assert_eq!(out, expected);
        }
    }
}
