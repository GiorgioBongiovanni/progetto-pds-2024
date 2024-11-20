use scrap::{Capturer, Display};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use bincode;

pub fn capture_and_send_frames(caster_address: &str) -> std::io::Result<()> {
    // Configura il socket TCP per accettare connessioni
    let listener = TcpListener::bind(caster_address)?;
    println!("Caster running on {}", caster_address);

    // Ottieni le informazioni sul display primario (per logging)
    let display = Display::primary().expect("Failed to get primary display");
    let (width, height) = (display.width(), display.height());
    println!("Screen size: {}x{}", width, height);

    // Accetta connessioni dai receiver
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr()?);

                // Avvia un thread per ogni receiver
                thread::spawn(move || {
                    if let Err(e) = handle_receiver(stream) {
                        eprintln!("Receiver error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

// Gestisce la comunicazione con un receiver
fn handle_receiver(mut stream: TcpStream) -> std::io::Result<()> {
    // Ottieni un nuovo `Display` per il receiver
    let display = Display::primary().expect("Failed to get primary display");
    let mut capturer = Capturer::new(display).expect("Failed to create capturer");
    let (width, height) = (capturer.width(), capturer.height());
    println!(
        "Capturer initialized for receiver at {}, screen size: {}x{}",
        stream.peer_addr()?,
        width,
        height
    );

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let frame_data = frame.to_vec();
                let data = (width, height, frame_data);

                // Serializza il frame
                let serialized_frame = bincode::serialize(&data).expect("Failed to serialize frame");

                // Invia la lunghezza del frame seguita dai dati
                println!("{}", serialized_frame.len());
                stream.write_all(&(serialized_frame.len() as u32).to_be_bytes())?;
                stream.write_all(&serialized_frame)?;

                // Simula 30 FPS
                thread::sleep(Duration::from_millis(33));
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                eprintln!("Error capturing frame: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}









