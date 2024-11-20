use std::io::Read;
use std::net::TcpStream;
use bincode;
use minifb::{Window, WindowOptions};

pub fn receive_frames(caster_address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(caster_address)?;
    println!("Connected to caster at {}", caster_address);

    // Variabili per la finestra di visualizzazione
    let mut window: Option<Window> = None;

    loop {
        // Legge la lunghezza del frame
        let mut length_buffer = [0u8; 4];
        stream.read_exact(&mut length_buffer)?;
        let frame_length = u32::from_be_bytes(length_buffer) as usize;
        // println!("Frame length: {}", frame_length);

        // Legge i dati del frame
        let mut frame_data = vec![0u8; frame_length];
        stream.read_exact(&mut frame_data)?;

        // Deserializza il frame ricevuto
        let (width, height, frame): (usize, usize, Vec<u8>) =
            bincode::deserialize(&frame_data).expect("Failed to deserialize frame");

        println!(
            "Frame received: {}x{}, pixels: {}",
            width, height, frame.len()
        );

        // Inizializza la finestra una sola volta
        if window.is_none() {
            window = Some(
                Window::new(
                    "Screen Casting Receiver",
                    width,
                    height,
                    WindowOptions::default(),
                )
                    .expect("Failed to create window"),
            );
        }

        if let Some(win) = &mut window {
            // Converte il frame in formato RGB (minifb utilizza u32 per ogni pixel in formato ARGB)
            let mut framebuffer: Vec<u32> = Vec::with_capacity(width * height);
            for chunk in frame.chunks(4) {
                // BGRA -> ARGB per minifb
                let b = chunk[0] as u32;
                let g = chunk[1] as u32;
                let r = chunk[2] as u32;
                framebuffer.push((0xFF << 24) | (r << 16) | (g << 8) | b);
            }

            // Mostra il frame nella finestra
            win.update_with_buffer(&framebuffer, width, height)
                .expect("Failed to update window");

            // Chiudi la finestra se l'utente la chiude
            if !win.is_open() {
                println!("Window closed. Exiting...");
                break;
            }
        }
    }

    Ok(())
}



