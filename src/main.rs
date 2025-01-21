use image::{GenericImageView, imageops::FilterType, ImageFormat};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Cargar la imagen en formato PNG
    let img = image::open("favicon.png")?;
    
    // Validar las dimensiones originales de la imagen
    let dimensions = img.dimensions();
    println!("Dimensiones originales de la imagen: {}x{}", dimensions.0, dimensions.1);

    // Lista de tamaños deseados para la imagen ICO
    let sizes = [16, 32, 48, 64, 128, 256]; // Max 256px

    for &size in &sizes {
        // Redimensionar la imagen al tamaño actual
        let resized_img = img.resize(size, size, FilterType::Lanczos3);

        // Nombre del archivo basado en el tamaño
        let filename = format!("favicon_{}x{}.ico", size, size);

        // Guardar la imagen redimensionada en formato ICO
        resized_img.save_with_format(&filename, ImageFormat::Ico)?;

        println!("Imagen guardada: {}", filename);
    }

    println!("Conversión completada para todos los tamaños.");

    Ok(())
}

// Pruebas

/* 
use image::{ImageFormat, GenericImageView};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Cargar la imagen en formato PNG
    let img = image::open("favicon.png")?;

    // Validar que sea una imagen PNG
    let dimensions = img.dimensions();
    println!("Dimensiones de la imagen: {}x{}", dimensions.0, dimensions.1);

    // Guardar la imagen en formato ICO
    img.save_with_format("favicon.ico", ImageFormat::Ico)?;

    println!("Conversión completada: favicon.png -> favicon.ico");

    Ok(())
} 
*/


/* 
use image::{ImageFormat, GenericImageView, imageops::FilterType};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Cargar la imagen en formato PNG
    let img = image::open("favicon.png")?;
    
    // Validar que sea una imagen PNG y obtener las dimensiones originales
    let dimensions = img.dimensions();
    println!("Dimensiones originales de la imagen: {}x{}", dimensions.0, dimensions.1);

    // Redimensionar la imagen a 16x16
    let resized_img = img.resize(16, 16, FilterType::Lanczos3);

    // Guardar la imagen redimensionada en formato ICO
    resized_img.save_with_format("favicon.ico", ImageFormat::Ico)?;

    println!("Conversión completada y redimensionada: favicon.png -> favicon.ico (16x16)");

    Ok(())
} 
*/