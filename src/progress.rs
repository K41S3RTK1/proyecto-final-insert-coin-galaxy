//! Versioned, validated save data; atomic replacement prevents half-written saves.
use std::{
    fs, io,
    path::{Path, PathBuf},
};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Progress {
    pub runner_score: u32,
    pub medals: [bool; 5],
    pub best_ms: [Option<u32>; 5],
}
impl Progress {
    pub fn award(&mut self, index: usize, seconds: f32) {
        self.medals[index] = true;
        let ms = (seconds.max(0.) * 1000.).round() as u32;
        self.best_ms[index] = Some(self.best_ms[index].map_or(ms, |old| old.min(ms)));
    }
    pub fn encode(&self) -> String {
        format!(
            "ARCADE_GALAXY 4\nmedals={}\ngarden_ms={}\ndojo_ms={}\nrunner_ms={}\nmario_ms={}\nrace_ms={}\nrunner_score={}\n",
            u8::from(self.medals[0])
                + 2 * u8::from(self.medals[1])
                + 4 * u8::from(self.medals[2])
                + 8 * u8::from(self.medals[3])
                + 16 * u8::from(self.medals[4]),
            self.best_ms[0].unwrap_or(0),
            self.best_ms[1].unwrap_or(0),
            self.best_ms[2].unwrap_or(0),
            self.best_ms[3].unwrap_or(0),
            self.best_ms[4].unwrap_or(0),
            self.runner_score
        )
    }

    pub fn decode(text: &str) -> Result<Self, String> {
        let mut lines = text.lines();
        let version = match lines.next() {
            Some("ARCADE_GALAXY 1") => 1,
            Some("ARCADE_GALAXY 2") => 2,
            Some("ARCADE_GALAXY 3") => 3,
            Some("ARCADE_GALAXY 4") => 4,
            _ => return Err("Version de guardado desconocida".into()),
        };
        let mut fields = std::collections::HashMap::new();
        for line in lines {
            let (k, v) = line.split_once('=').ok_or("Linea de guardado invalida")?;
            if fields.insert(k, v).is_some() {
                return Err("Campo duplicado".into());
            }
        }
        let number = |key| -> Result<u32, String> {
            fields
                .get(key)
                .ok_or_else(|| format!("Falta {key}"))?
                .parse()
                .map_err(|_| format!("Numero invalido: {key}"))
        };
        let medals = number("medals")?;
        if medals > (1 << (version + 1)) - 1 {
            return Err("Insignias invalidas".into());
        }
        let mut result = Self {
            runner_score: if version >= 2 {
                number("runner_score")?
            } else {
                0
            },
            medals: [
                medals & 1 != 0,
                medals & 2 != 0,
                medals & 4 != 0,
                medals & 8 != 0,
                medals & 16 != 0,
            ],
            best_ms: [None; 5],
        };
        for (i, key) in ["garden_ms", "dojo_ms", "runner_ms", "mario_ms", "race_ms"]
            .iter()
            .take(version + 1)
            .enumerate()
        {
            let ms = number(key)?;
            if ms > 600_000 {
                return Err("Tiempo invalido".into());
            }
            if ms > 0 {
                result.best_ms[i] = Some(ms)
            }
        }
        Ok(result)
    }
}
pub fn load(path: &Path) -> Result<Progress, String> {
    match fs::read_to_string(path) {
        Ok(text) => Progress::decode(&text),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Progress::default()),
        Err(e) => Err(e.to_string()),
    }
}
pub fn save(path: &Path, progress: &Progress) -> io::Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    let temporary = PathBuf::from(format!("{}.{}.tmp", path.display(), std::process::id()));
    let result = (|| {
        use std::io::Write;
        let mut file = fs::File::create(&temporary)?;
        file.write_all(progress.encode().as_bytes())?;
        file.sync_all()?;
        fs::rename(&temporary, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn badges_and_records_survive_restart() {
        let p = std::env::temp_dir().join(format!("arcade-progress-{}.txt", std::process::id()));
        let mut progress = Progress::default();
        progress.award(0, 18.125);
        progress.award(1, 34.5);
        progress.award(0, 20.);
        save(&p, &progress).unwrap();
        assert_eq!(load(&p).unwrap(), progress);
        assert_eq!(progress.best_ms[0], Some(18125));
        progress.award(0, 17.);
        save(&p, &progress).unwrap();
        assert_eq!(load(&p).unwrap().best_ms[0], Some(17000));
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn v2_save_preserves_runner_and_adds_fourth_medal() {
        let old = "ARCADE_GALAXY 2\nmedals=7\ngarden_ms=18000\ndojo_ms=34000\nrunner_ms=65000\nrunner_score=3200\n";
        let mut p = Progress::decode(old).unwrap();
        assert_eq!(p.medals, [true, true, true, false, false]);
        assert_eq!(p.runner_score, 3200);
        assert_eq!(p.best_ms[2], Some(65000));
        p.award(3, 72.5);
        let restored = Progress::decode(&p.encode()).unwrap();
        assert_eq!(restored, p);
        assert_eq!(restored.medals, [true, true, true, true, false]);
    }
    #[test]
    fn v3_preserves_four_worlds_and_adds_race_record() {
        let old = "ARCADE_GALAXY 3\nmedals=15\ngarden_ms=18000\ndojo_ms=34000\nrunner_ms=65000\nmario_ms=72500\nrunner_score=3200\n";
        let mut p = Progress::decode(old).unwrap();
        assert_eq!(p.medals, [true, true, true, true, false]);
        assert_eq!(p.best_ms[3], Some(72500));
        p.award(4, 39.5);
        assert_eq!(Progress::decode(&p.encode()).unwrap(), p);
        assert_eq!(p.best_ms[4], Some(39500));
    }
    #[test]
    fn invalid_save_is_reported() {
        for text in [
            "",
            "ARCADE_GALAXY 2",
            "ARCADE_GALAXY 1\nmedals=9\ngarden_ms=1\ndojo_ms=2",
            "ARCADE_GALAXY 1\nmedals=0\nmedals=1\ngarden_ms=0\ndojo_ms=0",
        ] {
            assert!(Progress::decode(text).is_err());
        }
    }
    #[test]
    fn v1_save_migrates_without_losing_existing_badges() {
        let old = "ARCADE_GALAXY 1\nmedals=3\ngarden_ms=18000\ndojo_ms=34000\n";
        let mut p = Progress::decode(old).unwrap();
        assert_eq!(p.medals, [true, true, false, false, false]);
        assert_eq!(p.best_ms, [Some(18000), Some(34000), None, None, None]);
        p.award(2, 60.);
        p.runner_score = 3500;
        assert_eq!(Progress::decode(&p.encode()).unwrap(), p);
    }
}
