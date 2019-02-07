use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub fn load_attrib(ctx: &WebGl2RenderingContext, data: &js_sys::Object, size: i32, data_type: u32, normalized: bool) -> Result<(), String> {
    let buffer = ctx.create_buffer().ok_or("Failed to create WebGlBuffer")?;

    ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    ctx.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &data,
        WebGl2RenderingContext::STATIC_DRAW
    );
    ctx.vertex_attrib_pointer_with_i32(0, size, data_type, normalized, 0, 0);
    ctx.enable_vertex_attrib_array(0);

    Ok(())
}

pub fn compile_shader(ctx: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = ctx.create_shader(shader_type).ok_or("Failed to create WebGlShader")?;

    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);

    let success = ctx.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool().unwrap_or(false);

    if success {
        Ok(shader)
    }
    else {
        Err(ctx.get_shader_info_log(&shader)
            .unwrap_or("Unknown error creating WebGlShader".to_owned()))
    }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(ctx: &WebGl2RenderingContext, shaders: T) -> Result<WebGlProgram, String> {
    let program = ctx.create_program().ok_or("Failed to create WebGlProgram")?;

    for shader in shaders {
        ctx.attach_shader(&program, shader)
    }

    ctx.link_program(&program);

    let success = ctx.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool().unwrap_or(false);

    if success {
        Ok(program)
    }
    else {
        Err(ctx.get_program_info_log(&program)
            .unwrap_or("Unknown error creating WebGlProgram".to_owned()))
    }
}