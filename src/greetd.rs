use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Request<'a> {
    CreateSession { username: &'a str },
    PostAuthMessageResponse { response: Option<&'a str> },
    StartSession { cmd: Vec<&'a str> },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Success,
    Error {
        error_type: String,
        description: String,
    },
    AuthMessage {
        auth_message_type: String,
        auth_message: String,
    },
}

pub struct GreetdClient {
    stream: UnixStream,
}

impl GreetdClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let socket_path = env::var("GREETD_SOCK")?;
        let stream = UnixStream::connect(socket_path)?;
        Ok(Self { stream })
    }

    fn send_request<T: Serialize>(&mut self, req: &T) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_vec(req)?;
        let len = (json.len() as u32).to_le_bytes();
        self.stream.write_all(&len)?;
        self.stream.write_all(&json)?;
        Ok(())
    }

    fn read_response(&mut self) -> Result<Response, Box<dyn std::error::Error>> {
        let mut len_buf = [0u8; 4];
        self.stream.read_exact(&mut len_buf)?;
        let len = u32::from_le_bytes(len_buf) as usize;

        let mut buf = vec![0u8; len];
        self.stream.read_exact(&mut buf)?;
        let resp: Response = serde_json::from_slice(&buf)?;
        Ok(resp)
    }

    pub fn login(&mut self, user: &str, pass: &str, cmd: Vec<&str>) -> Result<(), String> {
        // 1. Создаем сессию
        self.send_request(&Request::CreateSession { username: user })
            .map_err(|e| e.to_string())?;

        match self.read_response().map_err(|e| e.to_string())? {
            Response::AuthMessage { .. } => {}
            Response::Error { description, .. } => return Err(description),
            _ => {}
        }

        // 2. Отправляем пароль
        self.send_request(&Request::PostAuthMessageResponse {
            response: Some(pass),
        })
        .map_err(|e| e.to_string())?;

        match self.read_response().map_err(|e| e.to_string())? {
            Response::Success => {}
            Response::Error { description, .. } => return Err(description),
            _ => {}
        }

        // 3. Стартуем Hyprland
        self.send_request(&Request::StartSession { cmd })
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
