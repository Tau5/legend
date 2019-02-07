use std::path::Path;

pub fn get_path(path_from_cwd: String)-> String{ //Get the path where the executable is located
        let filen =  path_from_cwd;
        let path = std::env::current_exe().unwrap();
        
        format!("{}{}", path.parent().unwrap().display(), filen)
}
pub fn check_legend_dirs(){
    if !(Path::new(&format!("{}/legend", dirs::home_dir().unwrap().display())).exists()) {
        std::fs::create_dir(Path::new(&format!("{}/legend", dirs::home_dir().unwrap().display()))).expect("Can't create legend directory. This is a fatal error, check the permissions to resolve this error.");
    }
    if !(Path::new(&format!("{}/legend/saves", dirs::home_dir().unwrap().display())).exists()) {
        std::fs::create_dir(Path::new(&format!("{}/legend/saves", dirs::home_dir().unwrap().display()))).expect("Can't create saves directory. This is a fatal error, check the permissions to resolve this error.");
    }
}