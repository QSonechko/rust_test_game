use std::io::prelude::*;
use std::fs::File;

pub struct ShaderContainer {
    pub shaders: Vec<String>
}

impl ShaderContainer {
    pub fn new(shader_paths: Vec<&'static str>) -> ShaderContainer {
        let mut strings: Vec<String> = Vec::new();
        let mut string_shader = String::new();
        for shader_path in shader_paths {
            let mut shader_file = match File::open(shader_path) {
                Ok(file) => file,
                Err(_) => panic!("Failed to load shader {}", shader_path),
            };
            match shader_file.read_to_string(& mut string_shader) {
                Ok(_) => strings.push(string_shader.clone()),
                Err(_) => panic!("Failed to read {}", shader_path),
            };
        }
        
        return ShaderContainer { shaders: strings };
    }

    // TODO: find a way to return a pointer for this shit. 
    // For now though we'll just make it public.
    /* pub fn get_shaders(&self) -> Vec<String> {
        self.shaders
    } */
}
