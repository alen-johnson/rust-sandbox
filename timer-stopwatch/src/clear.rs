use std::process::Command;

pub fn clear(){
 // cross platform clear screen
 if cfg!(target_os = "windows") {
    Command::new("cmd").arg("/C").arg("cls").status().expect("Failed to clear screen");
} else {
    Command::new("clear").status().expect("Failed to clear screen");
}
}