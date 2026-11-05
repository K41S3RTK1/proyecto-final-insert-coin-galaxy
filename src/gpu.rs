//! GPU rasterization of the same CPU-authored meshes and model/view/projection.
//! Full-resolution color/depth, per-fragment lighting, smooth font atlas and AA.
use crate::{
    game::{Game, Mode, WorldId},
    renderer::{GpuVertex, Paint, Renderer},
};
use raylib::{ffi, prelude::*};
use std::{error::Error, path::Path};
pub struct Gpu {
    shader: Shader,
    smooth: Shader,
    target: RenderTexture2D,
    resolved: RenderTexture2D,
    font: Font,
    vao: u32,
    vbo: u32,
    capacity: usize,
    width: usize,
    height: usize,
}
impl Gpu {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        width: usize,
        height: usize,
        assets: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        let shader = rl.load_shader_from_memory(
            thread,
            Some(include_str!("../shaders/scene.vs")),
            Some(include_str!("../shaders/scene.fs")),
        );
        let smooth =
            rl.load_shader_from_memory(thread, None, Some(include_str!("../shaders/smooth.fs")));
        for s in [&shader, &smooth] {
            if s.as_ref().id == unsafe { ffi::rlGetShaderIdDefault() } {
                return Err("No se pudo compilar el shader HD".into());
            }
        }
        let font = rl.load_font_ex(
            thread,
            &assets.join("fonts/Nunito.ttf").to_string_lossy(),
            64,
            None,
        )?;
        font.texture()
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        let target = rl.load_render_texture(thread, width as u32, height as u32)?;
        target
            .texture()
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        let resolved = rl.load_render_texture(thread, width as u32, height as u32)?;
        resolved
            .texture()
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        Ok(Self {
            shader,
            smooth,
            target,
            resolved,
            font,
            vao: 0,
            vbo: 0,
            capacity: 0,
            width,
            height,
        })
    }
    pub fn resize(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        w: usize,
        h: usize,
    ) -> Result<(), Box<dyn Error>> {
        self.target = rl.load_render_texture(thread, w as u32, h as u32)?;
        self.target
            .texture()
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        self.resolved = rl.load_render_texture(thread, w as u32, h as u32)?;
        self.resolved
            .texture()
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        self.width = w;
        self.height = h;
        Ok(())
    }
    pub fn render(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        scene: &Renderer,
        hud: &Renderer,
        game: &Game,
    ) {
        let bytes = scene.gpu_vertices.len() * std::mem::size_of::<GpuVertex>();
        unsafe {
            ffi::rlDrawRenderBatchActive();
            if bytes > self.capacity {
                if self.vao != 0 {
                    ffi::rlUnloadVertexArray(self.vao);
                    ffi::rlUnloadVertexBuffer(self.vbo);
                }
                self.capacity = bytes.next_power_of_two().max(4096);
                self.vao = ffi::rlLoadVertexArray();
                ffi::rlEnableVertexArray(self.vao);
                self.vbo = ffi::rlLoadVertexBuffer(std::ptr::null(), self.capacity as i32, true);
                ffi::rlEnableVertexBuffer(self.vbo);
                for (index, size, offset) in [
                    (0, 4, 0),
                    (1, 3, 16),
                    (2, 3, 28),
                    (3, 4, 40),
                    (4, 3, 56),
                    (5, 1, 68),
                ] {
                    ffi::rlSetVertexAttribute(index, size, 0x1406, false, 72, offset);
                    ffi::rlEnableVertexAttribute(index);
                }
                ffi::rlDisableVertexBuffer();
                ffi::rlDisableVertexArray();
            }
            if bytes > 0 {
                ffi::rlUpdateVertexBuffer(
                    self.vbo,
                    scene.gpu_vertices.as_ptr().cast(),
                    bytes as i32,
                    0,
                );
            }
        }
        let fog = if game.inside() {
            match game.active_world {
                WorldId::Garden => Vector3::new(0.63, 0.77, 0.67),
                WorldId::Dojo => Vector3::new(0.18, 0.16, 0.30),
                _ => Vector3::new(0.48, 0.76, 0.9),
            }
        } else {
            Vector3::new(0.05, 0.08, 0.16)
        };
        self.shader.set_shader_value(
            self.shader.get_shader_location("eye"),
            Vector3::new(scene.eye.x, scene.eye.y, scene.eye.z),
        );
        self.shader
            .set_shader_value(self.shader.get_shader_location("time"), scene.time);
        self.shader
            .set_shader_value(self.shader.get_shader_location("fogColor"), fog);
        self.shader.set_shader_value(
            self.shader.get_shader_location("fogAmount"),
            if game.inside() { 0.45_f32 } else { 0. },
        );
        let mut d = rl.begin_texture_mode(thread, &mut self.target);
        d.clear_background(Color::BLACK);
        draw_paint(&mut d, &self.font, &scene.background, 1.);
        unsafe {
            ffi::rlDrawRenderBatchActive();
            ffi::rlEnableDepthTest();
            ffi::rlDisableBackfaceCulling();
            ffi::rlEnableShader(self.shader.as_ref().id);
            if bytes > 0 {
                ffi::rlEnableVertexArray(self.vao);
                ffi::rlDrawVertexArray(0, scene.gpu_vertices.len() as i32);
                ffi::rlDisableVertexArray();
            }
            ffi::rlDisableShader();
            ffi::rlDisableDepthTest();
            ffi::rlEnableBackfaceCulling();
        }
        draw_paint(&mut d, &self.font, &scene.overlay, 1.);
        let fade = if game.paused {
            0.
        } else if game.mode == Mode::Entering {
            ((0.32 - game.portal_left) / 0.32).clamp(0., 1.)
        } else if game.inside() {
            ((game.arrival - 2.8) / 0.6).clamp(0., 1.)
        } else {
            (game.arrival / 0.7).clamp(0., 1.)
        };
        if fade > 0. {
            d.draw_rectangle(
                0,
                0,
                self.width as i32,
                self.height as i32,
                Color::new(233, 220, 183, (fade * 255.) as u8),
            );
        }
        drop(d);
        self.smooth.set_shader_value(
            self.smooth.get_shader_location("texel"),
            Vector2::new(1. / self.width as f32, 1. / self.height as f32),
        );
        let mut output = rl.begin_texture_mode(thread, &mut self.resolved);
        output.clear_background(Color::BLACK);
        let mut aa = output.begin_shader_mode(&mut self.smooth);
        aa.draw_texture_pro(
            self.target.texture(),
            Rectangle::new(0., 0., self.width as f32, -(self.height as f32)),
            Rectangle::new(0., 0., self.width as f32, self.height as f32),
            Vector2::zero(),
            0.,
            Color::WHITE,
        );
        drop(aa);
        draw_paint(
            &mut output,
            &self.font,
            &hud.background,
            self.width as f32 / 640.,
        );
        draw_paint(
            &mut output,
            &self.font,
            &hud.overlay,
            self.width as f32 / 640.,
        );
    }
    pub fn present(&self, d: &mut RaylibDrawHandle, rect: Rectangle) {
        d.draw_texture_pro(
            self.resolved.texture(),
            Rectangle::new(0., 0., self.width as f32, -(self.height as f32)),
            rect,
            Vector2::zero(),
            0.,
            Color::WHITE,
        );
    }
    pub fn rgba8(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut img = self.resolved.texture().load_image()?;
        img.flip_vertical();
        let colors = img.get_image_data();
        Ok(colors.iter().flat_map(|c| [c.r, c.g, c.b, c.a]).collect())
    }
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut img = self.resolved.texture().load_image()?;
        img.flip_vertical();
        img.export_image(&path.to_string_lossy());
        Ok(())
    }
}
impl Drop for Gpu {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                ffi::rlUnloadVertexArray(self.vao);
                ffi::rlUnloadVertexBuffer(self.vbo);
            }
        }
    }
}
fn draw_paint(d: &mut impl RaylibDraw, font: &Font, commands: &[Paint], scale: f32) {
    for cmd in commands {
        match cmd {
            Paint::Rect(x, y, w, h, c) => d.draw_rectangle_rec(
                Rectangle::new(
                    *x as f32 * scale,
                    *y as f32 * scale,
                    *w as f32 * scale,
                    *h as f32 * scale,
                ),
                Color::new(c[0], c[1], c[2], 255),
            ),
            Paint::Line(a, b, c) => d.draw_line_ex(
                Vector2::new(a.0 as f32 * scale, a.1 as f32 * scale),
                Vector2::new(b.0 as f32 * scale, b.1 as f32 * scale),
                scale.max(1.),
                Color::new(c[0], c[1], c[2], 255),
            ),
            Paint::Text(x, y, text, size, c) => {
                // Keep the logical UI layout, but rasterize a 64px TrueType atlas.
                let font_size = *size as f32 * 10. * scale;
                for (k, ch) in text.chars().enumerate() {
                    d.draw_text_ex(
                        font,
                        &ch.to_string(),
                        Vector2::new(
                            (*x as f32 + k as f32 * 6. * (*size as f32)) * scale,
                            (*y as f32 - 1.5 * (*size as f32)) * scale,
                        ),
                        font_size,
                        0.,
                        Color::new(c[0], c[1], c[2], 255),
                    );
                }
            }
        }
    }
}
