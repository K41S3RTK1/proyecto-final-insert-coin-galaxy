use crate::math::Vec3;
use std::{collections::HashMap, fs, path::Path};
#[derive(Debug)]
pub struct Face {
    pub vertices: [usize; 3],
    pub material: String,
    pub normals: [Option<usize>; 3],
}
pub struct Model {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub radius: f32,
    pub faces: Vec<Face>,
    pub colors: HashMap<String, [u8; 3]>,
}
impl Model {
    /// OBJ se indexa desde 1; los índices negativos son relativos al final.
    pub fn parse(source: &str) -> Result<Self, String> {
        let mut model = Self {
            vertices: vec![],
            normals: vec![],
            radius: 0.,
            faces: vec![],
            colors: HashMap::new(),
        };
        let mut material = String::new();
        for (line_no, line) in source.lines().enumerate() {
            let words: Vec<_> = line
                .split('#')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect();
            if words.is_empty() {
                continue;
            }
            let error = || format!("OBJ línea {}: datos inválidos", line_no + 1);
            match words[0] {
                "v" | "vn" => {
                    if words.len() < 4 {
                        return Err(error());
                    }
                    let values: Result<Vec<f32>, _> =
                        words[1..4].iter().map(|v| v.parse::<f32>()).collect();
                    let v = values.map_err(|_| error())?;
                    if !v.iter().all(|n| n.is_finite()) {
                        return Err(error());
                    }
                    let vector = Vec3::new(v[0], v[1], v[2]);
                    if words[0] == "vn" {
                        if vector.length() < 1e-8 {
                            return Err(error());
                        }
                        model.normals.push(vector.normalized());
                    } else {
                        model.radius = model.radius.max(vector.length());
                        model.vertices.push(vector);
                    }
                }
                "usemtl" => {
                    material = words[1..].join(" ");
                }
                "f" => {
                    if words.len() != 4 {
                        return Err(format!(
                            "OBJ línea {}: se requieren caras trianguladas",
                            line_no + 1
                        ));
                    }
                    let mut indices = [0; 3];
                    let mut normals = [None; 3];
                    for (i, token) in words[1..].iter().enumerate() {
                        let raw = token
                            .split('/')
                            .next()
                            .unwrap()
                            .parse::<i64>()
                            .map_err(|_| error())?;
                        let index = if raw > 0 {
                            raw - 1
                        } else {
                            model.vertices.len() as i64 + raw
                        };
                        if raw == 0 || index < 0 || index >= model.vertices.len() as i64 {
                            return Err(error());
                        }
                        indices[i] = index as usize;
                        if let Some(normal) = token.split('/').nth(2).filter(|s| !s.is_empty()) {
                            let raw = normal.parse::<i64>().map_err(|_| error())?;
                            let index = if raw > 0 {
                                raw - 1
                            } else {
                                model.normals.len() as i64 + raw
                            };
                            if raw == 0 || index < 0 || index >= model.normals.len() as i64 {
                                return Err(error());
                            }
                            normals[i] = Some(index as usize);
                        }
                    }
                    model.faces.push(Face {
                        vertices: indices,
                        normals,
                        material: material.clone(),
                    });
                }
                _ => {}
            }
        }
        if model.vertices.is_empty() || model.faces.is_empty() {
            return Err("OBJ sin geometría triangular".into());
        }
        Ok(model)
    }
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let source = fs::read_to_string(path)?;
        let mut model = Self::parse(&source)?;
        for line in source.lines() {
            if let Some(name) = line.trim().strip_prefix("mtllib ") {
                let mtl =
                    fs::read_to_string(path.parent().unwrap_or(Path::new(".")).join(name.trim()))?;
                let mut current = String::new();
                for line in mtl.lines() {
                    let p: Vec<_> = line.split_whitespace().collect();
                    if p.first() == Some(&"newmtl") {
                        current = p[1..].join(" ");
                    }
                    if p.first() == Some(&"Kd") && p.len() >= 4 {
                        let mut color = [0; 3];
                        for i in 0..3 {
                            let v = p[i + 1].parse::<f32>()?;
                            if !v.is_finite() {
                                return Err("MTL: color no finito".into());
                            }
                            color[i] = (v.clamp(0.0, 1.0) * 255.0).round() as u8;
                        }
                        model.colors.insert(current.clone(), color);
                    }
                }
            }
        }
        Ok(model)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn obj_indices_y_formatos() {
        let m = Model::parse("v 0 0 0\nv 1 0 0\nv 0 1 0\nvn 0 0 1\nf 1/2/1 2//1 3\nf -3 -2 -1")
            .unwrap();
        assert_eq!(m.faces[0].vertices, [0, 1, 2]);
        assert_eq!(m.faces[1].vertices, [0, 1, 2]);
    }
    #[test]
    fn bundled_models_have_materials_normals_and_smaller_distance_meshes() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let ship = Model::load(&root.join("arcade-ship.obj")).unwrap();
        assert!(ship.faces.len() > 4000);
        for name in ["garden", "dojo", "runner", "mario", "race"] {
            let near = Model::load(&root.join(format!("models/{name}.obj"))).unwrap();
            let far = Model::load(&root.join(format!("models/{name}-low.obj"))).unwrap();
            assert!(far.faces.len() < near.faces.len());
            assert!(near.radius > 5. && near.radius < 10.);
            for model in [&near, &far, &ship] {
                assert!(
                    model
                        .faces
                        .iter()
                        .all(|f| f.normals.iter().all(Option::is_some)
                            && model.colors.contains_key(&f.material))
                );
                assert!(
                    model
                        .normals
                        .iter()
                        .all(|n| (n.length() - 1.).abs() < 0.0001)
                );
            }
            let mut r = crate::renderer::Renderer::new(96, 96);
            r.begin(Vec3::new(10., 8., 17.), Vec3::default(), 0.);
            r.model(&near, Vec3::default(), 1., 0.);
            assert!(
                r.stats.fragments > 1000,
                "{name} must rasterize in software"
            );
        }
    }
    #[test]
    fn normal_indices_are_validated_and_negative_indices_work() {
        let source = "v 0 0 0\nv 1 0 0\nv 0 1 0\nvn 0 0 3\nf 1//-1 2//1 3//1";
        let m = Model::parse(source).unwrap();
        assert_eq!(m.faces[0].normals, [Some(0); 3]);
        assert!((m.normals[0].length() - 1.).abs() < 0.0001);
        assert_eq!(m.radius, 1.);
        for index in ["0", "2", "-2"] {
            assert!(Model::parse(&source.replace("1//-1", &format!("1//{index}"))).is_err());
        }
        assert!(Model::parse(&source.replace("vn 0 0 3", "vn 0 0 0")).is_err());
    }
    #[test]
    fn rechaza_indices_y_poligonos_invalidos() {
        let base = "v 0 0 0\nv 1 0 0\nv 0 1 0\n";
        for f in ["f 0 2 3", "f 1 2 4", "f -4 -2 -1", "f 1 2 3 1"] {
            assert!(Model::parse(&format!("{base}{f}")).is_err())
        }
    }
}
