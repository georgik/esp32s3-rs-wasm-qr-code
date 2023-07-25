use qrcode::{QrCode, EcLevel};
use qrcode::render::unicode;
use console::Term;

fn main() {
    // The data to encode
    let data = "https://github.com/esp-rs";

    // Generate the QR Code of this data
    let code = QrCode::with_error_correction_level(data, EcLevel::M).unwrap();

    // Create a string to hold our QR code
    let qr_string = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();

    // Use the console crate to print out the QR code to the terminal
    let term = Term::stdout();
    term.write_line(&qr_string).unwrap();
}
