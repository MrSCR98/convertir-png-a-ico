# Conversión de Imágenes PNG a ICO (Favicons)

Este proyecto convierte un archivo de imagen PNG en un archivo de icono (ICO) con varias resoluciones, que es útil para favicon de sitios web. El programa redimensiona la imagen PNG a varios tamaños estándar (16x16, 32x32, 48x48, 64x64, 128x128 y 256x256) y guarda cada versión en formato .ico.

[**Descargar el ejecutable para Windows aquí !**](https://github.com/MrSCR98/convertir-png-a-ico/releases/download/Ejecutable.exe/convertir_png_a_ico.exe)

## Cómo usarlo

Sigue estos pasos para convertir un archivo PNG en íconos ICO de diferentes tamaños.

### Organización de Archivos

```
📁 Carpeta principal // Nueva carpeta para organizar los archivos
 ├── convertir_png_a_ico.exe   <-- Ejecutable descargado
 ├── favicon.png               <-- Imagen original (entrada)
 ├── favicon_16x16.ico         <-- Icono de 16x16 píxeles // generado
 ├── favicon_32x32.ico         <-- Icono de 32x32 píxeles // generado
 ├── favicon_48x48.ico         <-- Icono de 48x48 píxeles // generado
 ├── favicon_64x64.ico         <-- Icono de 64x64 píxeles // generado
 ├── favicon_128x128.ico       <-- Icono de 128x128 píxeles // generado
 └── favicon_256x256.ico       <-- Icono de 256x256 píxeles // generado
```

### Pasos

1. **Descarga el ejecutable**: 
   [Haz clic aquí para descargar el archivo](https://github.com/MrSCR98/convertir-png-a-ico/releases/download/Ejecutable.exe/convertir_png_a_ico.exe).
   
2. **Crea una carpeta**:
   - Crea una carpeta donde quieras trabajar y guarda el archivo \`convertir_png_a_ico.exe\` en ella.

3. **Coloca la imagen PNG**:
   - Añade tu imagen llamada \`favicon.png\` en la misma carpeta.

4. **Ejecuta el programa**:
   - Haz doble clic en \`convertir_png_a_ico.exe\`. El programa automáticamente procesará la imagen y generará los archivos ICO.

5. **Verifica los resultados**:
   - Los íconos generados (\`favicon_16x16.ico\`, \`favicon_32x32.ico\`, etc.) estarán en la misma carpeta.

---

### Resultado Final

El programa generará los archivos ICO con varios tamaños que puedes usar como favicons o en otros contextos.

---

## Requisitos

1. **Instalar Rust** (opcional): Si deseas compilar el código por ti mismo y realizar modificaciones, necesitas tener Rust instalado. Puedes instalar Rust desde [https://www.rust-lang.org](https://www.rust-lang.org).
   
   **Sin embargo, si solo deseas usar el programa**, puedes descargar el archivo ejecutable ya compilado, y no necesitas instalar Rust.

## Instalación de Dependencias (si decides compilar el código)

Si prefieres compilar el código y generar el ejecutable por ti mismo, sigue estos pasos:

### 1. Clonar el Repositorio

Primero, clona este repositorio en tu máquina local:

```cmd
git clone https://github.com/tu-usuario/conversion-favicon.git
cd conversion-favicon
```

### 2. Instalar Dependencias

El proyecto usa el crate `image`, que se incluye en el archivo `Cargo.toml` de este proyecto. Para instalar las dependencias y compilar el proyecto, ejecuta:

```cmd
cargo build
```

Este comando descargará e instalará todas las dependencias necesarias y generará el archivo ejecutable en el directorio `target/debug`.

## Uso del Proyecto

### 1. Preparar el archivo `favicon.png`

Asegúrate de tener una imagen en formato **PNG** que quieras convertir a un archivo ICO. Esta imagen debe llamarse `favicon.png` y debe estar ubicada en el **directorio raíz** del proyecto (donde está el archivo `Cargo.toml` o el ejecutable compilado).

### 2. Ejecutar el Proyecto

Con la imagen `favicon.png` en el directorio correcto, puedes ejecutar el proyecto de las siguientes maneras:

#### Si has compilado el código:
- Ejecuta el siguiente comando para compilar y ejecutar el programa:

```cmd
cargo run
```

Este comando generará los archivos `favicon_XxX.ico` en el mismo directorio.

#### Si ya tienes el ejecutable:
- Si ya has descargado el ejecutable compilado, simplemente pon el archivo **`favicon.png`** en la misma carpeta que el ejecutable, y luego ejecuta:

```cmd
./conversion-favicon
```

El programa generará los archivos `favicon_XxX.ico` en la misma carpeta.

### 3. Crear la Build (si decides compilar el código)

Si deseas crear una versión optimizada del programa para producción, puedes usar el siguiente comando:

```cmd
cargo build --release
```

Esto generará una versión optimizada del ejecutable en el directorio `target/release`. Puedes usar este ejecutable para realizar la conversión en otros sistemas sin necesidad de compilar el código cada vez.

## Estructura de Archivos

```
/conversion-favicon
├── Cargo.toml           # Archivo de configuración de dependencias y proyecto.
├── src/
│   └── main.rs          # Código fuente principal (el que convierte las imágenes).
├── favicon.png          # Tu imagen original (debe llamarse favicon.png).
├── target/              # Carpeta generada para las builds de Rust.
└── README.md            # Este archivo.
```

## Notas

- Si la imagen `favicon.png` no está en el directorio adecuado, el programa lanzará un error diciendo que no pudo encontrar el archivo.
- El programa genera un archivo .ico para cada tamaño de imagen. Estos archivos pueden ser utilizados como favicons para tu sitio web o aplicación.

**¡Listo! Ahora puedes usar el programa para crear favicons con facilidad!**

