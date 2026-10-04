pub fn login_user(buffer: &[u8]) {

    deserialize_login(buffer);
}

fn deserialize_login(buffer: &[u8]) {
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
}