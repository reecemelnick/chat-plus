use crate::database_manager;

pub struct RegisterUser {
    username: String,
    password: String,
}

impl RegisterUser {
    pub fn get_username(&self) -> String {
        return self.username.clone();
    }

    pub fn get_password(&self) -> String {
        return self.password.clone();
    }
}

pub fn register_user(buffer: &[u8]) {

    let new_user = deserialize_register(buffer);

    database_manager::register_user(&new_user);

}

fn deserialize_register(buffer: &[u8]) -> RegisterUser {
    let mut i: u16;

    let mut username: &str;
    let mut password: &str;

    i = 1;
    let username_length: [u8; 2] = buffer[i as usize..(i+2) as usize].try_into().expect("Slice was not 2 bytes long...");
    let username_length = u16::from_be_bytes(username_length);
    i += 2;

    username = str::from_utf8(&buffer[i as usize..(i+username_length) as usize]).unwrap();
    
    i += username_length;

    let password_length: [u8; 2] = buffer[i as usize..(i+2) as usize].try_into().expect("Slice was not 2 bytes long...");
    let password_length = u16::from_be_bytes(password_length);
    i += 2;

    password = str::from_utf8(&buffer[i as usize..(i + password_length) as usize]).unwrap();

    println!("New User...\nUsername: {}\nPassword: {}", username, password);

    RegisterUser {
        username: String::from(username),
        password: String::from(password),
    }
}