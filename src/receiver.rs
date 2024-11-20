use std::io::Read;
use std::net::TcpStream;
use bincode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn receive_frames(caster_address: &str) -> std::io::Result<()> {
    // Connessione al caster
    let mut stream = TcpStream::connect(caster_address)?;
    println!("Connected to caster at {}", caster_address);

    // Inizializza SDL2
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to initialize video subsystem");

    // Parametri iniziali per la finestra
    let width = 800; // Dimensioni iniziali di default
    let height = 600;

    // Creazione della finestra e del canvas
    let window = video_subsystem
        .window("Screen Casting Receiver", width, height)
        .resizable()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas().build().expect("Failed to create canvas");

    // Crea una texture per il rendering dei frame
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, width, height)
        .expect("Failed to create texture");

    // Ottieni il gestore degli eventi
    let mut event_pump = sdl_context.event_pump().expect("Failed to create event pump");

    loop {
        // Gestione degli eventi della finestra
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Exiting...");
                    return Ok(());
                }
                _ => {}
            }
        }

        // Legge la lunghezza del frame
        let mut length_buffer = [0u8; 4];
        stream.read_exact(&mut length_buffer)?;
        let frame_length = u32::from_be_bytes(length_buffer) as usize;

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

        // Ridimensiona la texture se necessario
        texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, width as u32, height as u32)
            .expect("Failed to resize texture");

        // Copia i dati del frame nella texture
        texture
            .with_lock(None, |buffer: &mut [u8], _| {
                buffer.copy_from_slice(&frame);
            })
            .expect("Failed to update texture");

        // Mostra la texture nella finestra
        canvas.clear();
        canvas.copy(&texture, None, None).expect("Failed to copy texture to canvas");
        canvas.present();
    }
}




